mod middleware;

use crate::util::ReturnedError;
use crate::{AnyhowBridge, ExtractedAppData, BASE64_ENGINE};
use actix_web::web::Json;
use actix_web::{post, web, Either, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Context;
use base64::Engine;
use entity::sea_orm_active_enums::KeyScope;
use entity::{api_key_scopes, api_keys, group_subscribers, groups, services, subscribers};
use migration::sea_query;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ActiveValue, IntoActiveModel, ModelTrait, PaginatorTrait, QueryTrait,
};
use sea_orm::{ColumnTrait, TransactionTrait};
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_push::{
    IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

fn bounce_bad_key<R>() -> Either<Json<ReturnedError>, R> {
    Either::Left(web::Json::<ReturnedError>("dup name".into()))
}

#[derive(Serialize, Deserialize, Debug)]
struct PostSubscribeBodyKeys {
    p256dh: String,
    auth: String,
}

#[derive(Deserialize, Debug)]
pub struct PostSubscribeSubscription {
    endpoint: String,
    keys: PostSubscribeBodyKeys,
    name: Option<String>,
    email: Option<String>,
}

#[derive(Deserialize, Debug)]
struct PostSubscribeBody {
    subscription: PostSubscribeSubscription,
    groups: Vec<Uuid>,
}

async fn key_has_scope(
    db: &sea_orm::DatabaseConnection,
    key: &[u8],
    service_id: &Uuid,
    group_id: &[Uuid],
    scope: KeyScope,
) -> crate::Result<bool> {
    let check = api_key_scopes::Entity::find()
        .inner_join(api_keys::Entity)
        .filter(
            api_keys::Column::ServiceId
                .eq(*service_id)
                .and(api_keys::Column::Key.eq(key))
                .and(api_key_scopes::Column::GroupId.is_null())
                .and(api_key_scopes::Column::Scope.eq(scope.clone())),
        )
        .count(db)
        .await?
        > 0
        || api_key_scopes::Entity::find()
            .inner_join(api_keys::Entity)
            .filter(
                api_keys::Column::ServiceId
                    .eq(*service_id)
                    .and(api_keys::Column::Key.eq(key))
                    .and(api_key_scopes::Column::GroupId.is_in(group_id.iter().copied()))
                    .and(api_key_scopes::Column::Scope.eq(scope.clone())),
            )
            .count(db)
            .await?
            == group_id.len() as u64;

    let mut api_key = api_keys::Entity::find()
        .filter(api_keys::Column::Key.eq(key))
        .one(db)
        .await
        .context("get key to update last_used")?
        .context("key to update last_used DNE")?
        .into_active_model();

    api_key.last_used =
        ActiveValue::Set(Some(sea_orm::sqlx::types::chrono::Utc::now().naive_utc()));
    api_key.update(db).await.context("push update last_used")?;

    Ok(check)
}

#[post("/service/{service_id}/subscribe")]
async fn subscribe(
    data: ExtractedAppData,
    auth: BearerAuth,
    service_id: web::Path<Uuid>,
    body: web::Json<PostSubscribeBody>,
) -> crate::Result<impl Responder> {
    let Ok(auth) = BASE64_ENGINE.decode(auth.token()) else {
        return Ok(bounce_bad_key());
    };

    let service_id = service_id.into_inner();
    let body = body.into_inner();

    if !key_has_scope(&data.db, &auth, &service_id, &body.groups, KeyScope::Sub).await? {
        // disguise this
        return Ok(bounce_bad_key());
    }

    // TODO: encrypt the endpoint and creds

    data.db
        .transaction::<_, (), AnyhowBridge>(move |txn| {
            Box::pin(async move {
                let endpoint = body.subscription.endpoint;

                let sub_id = match subscribers::Entity::find()
                    .filter(subscribers::Column::Endpoint.eq(endpoint.clone()))
                    .one(txn)
                    .await?
                {
                    Some(sub) => sub.subscriber_id,
                    None => {
                        let sub_id = Uuid::now_v7();

                        let ent = subscribers::ActiveModel {
                            subscriber_id: ActiveValue::set(sub_id),
                            name: ActiveValue::set(body.subscription.name),
                            email: ActiveValue::set(body.subscription.email),
                            endpoint: ActiveValue::set(endpoint),
                            client_key: ActiveValue::Set(
                                serde_json::to_string(&body.subscription.keys)
                                    .context("json encode keys")?,
                            ),
                        };

                        let sub_ids = subscribers::Entity::insert(ent)
                            .on_conflict(
                                sea_query::OnConflict::column(subscribers::Column::Endpoint)
                                    .update_column(subscribers::Column::Name)
                                    .update_column(subscribers::Column::Email)
                                    .update_column(subscribers::Column::ClientKey)
                                    .to_owned(),
                            )
                            .exec_with_returning_keys(txn)
                            .await
                            .context("insert new subscriber")?;

                        sub_ids[0]
                    }
                };

                group_subscribers::Entity::insert_many(
                    body.groups
                        .iter()
                        .map(|group_id| group_subscribers::ActiveModel {
                            service_id: ActiveValue::Set(service_id),
                            group_id: ActiveValue::Set(*group_id),
                            subscriber_id: ActiveValue::set(sub_id),
                        })
                        .collect::<Vec<_>>(),
                )
                .on_conflict(
                    OnConflict::columns([
                        group_subscribers::Column::ServiceId,
                        group_subscribers::Column::GroupId,
                        group_subscribers::Column::SubscriberId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec(txn)
                .await?;

                Ok(())
            })
        })
        .await?;

    Ok(Either::Right("ok"))
}

#[derive(Deserialize)]
pub struct PostNotifyBody {
    payload: String,
}

#[post("/service/{service_id}/group/{group_id}/notify")]
async fn notify(
    data: ExtractedAppData,
    auth: BearerAuth,
    params: web::Path<(Uuid, Uuid)>,
    body: web::Json<PostNotifyBody>,
) -> crate::Result<impl Responder> {
    let Ok(auth) = BASE64_ENGINE.decode(auth.token()) else {
        return Ok(bounce_bad_key());
    };

    let (service_id, group_id) = params.into_inner();
    let body = body.into_inner();

    if !key_has_scope(&data.db, &auth, &service_id, &[group_id], KeyScope::Notify).await? {
        // disguise this
        return Ok(bounce_bad_key());
    }

    let svc = services::Entity::find_by_id(service_id)
        .one(&data.db)
        .await
        .context("get service by id")?
        .context("no service by id")?;

    let mut group = groups::Entity::find_by_id((service_id, group_id))
        .one(&data.db)
        .await
        .context("get group to update last_notified")?
        .context("group to update last_notified DNE")?
        .into_active_model();

    group.last_notified =
        ActiveValue::Set(Some(sea_orm::sqlx::types::chrono::Utc::now().naive_utc()));
    group
        .update(&data.db)
        .await
        .context("push update last_notified")?;

    let page_size = 1_000;
    let mut it = group_subscribers::Entity::find()
        .filter(
            group_subscribers::Column::ServiceId
                .eq(service_id)
                .and(group_subscribers::Column::GroupId.eq(group_id)),
        )
        .find_also_related(subscribers::Entity)
        .paginate(&data.db, page_size);

    let content_bytes = body.payload.as_bytes();
    let client = IsahcWebPushClient::new().context("construct push client")?;

    let mut failed_pushes = Vec::with_capacity(page_size as usize);

    while let Some(ch) = it
        .fetch_and_next()
        .await
        .context("next page of subscribers")?
    {
        for sub in ch {
            let (group_sub, Some(sub)) = sub else {
                continue;
            };

            let Ok(keys) = serde_json::from_str::<PostSubscribeBodyKeys>(&sub.client_key) else {
                continue;
            };

            let subscription_info = SubscriptionInfo::new(sub.endpoint, keys.p256dh, keys.auth);

            let sig_builder =
                VapidSignatureBuilder::from_base64(&svc.vapid_private, &subscription_info)
                    .context("make sig builder")?
                    .build()
                    .context("build sig")?;

            let mut builder = WebPushMessageBuilder::new(&subscription_info);

            builder.set_payload(web_push::ContentEncoding::Aes128Gcm, content_bytes);
            builder.set_vapid_signature(sig_builder);

            // TODO: topic, urgency

            if let Err(_) = client
                .send(builder.build().context("build push message")?)
                .await
            {
                failed_pushes.push(group_sub.subscriber_id);
            }
        }

        group_subscribers::Entity::delete_many()
            .filter(
                group_subscribers::Column::ServiceId
                    .eq(service_id)
                    .and(group_subscribers::Column::GroupId.eq(group_id))
                    .and(group_subscribers::Column::SubscriberId.is_in(failed_pushes.drain(..))),
            )
            .exec(&data.db)
            .await
            .context("delete failed pushes")?;
    }

    Ok(Either::Right("ok"))
}
