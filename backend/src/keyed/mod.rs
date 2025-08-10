mod middleware;

use crate::{AnyhowBridge, ExtractedAppData};
use actix_web::http::StatusCode;
use actix_web::{Either, Responder, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Context;
use entity::sea_orm_active_enums::KeyScope;
use entity::{api_key_scopes, group_subscribers, services, subscribers};
use migration::sea_query;
use sea_orm::{ActiveValue, PaginatorTrait};
use sea_orm::{ColumnTrait, TransactionTrait};
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_push::{
    IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

#[derive(Serialize, Deserialize, Debug)]
struct PostSubscribeBodyKeys {
    p256dh: String,
    auth: String,
}

#[derive(Deserialize, Debug)]
pub struct PostSubscribeBody {
    endpoint: String,
    keys: PostSubscribeBodyKeys,
    name: Option<String>,
    email: Option<String>,
}

// TODO: set last_used
#[post("/service/{service_id}/group/{group_id}/subscribe")]
async fn subscribe(
    data: ExtractedAppData,
    auth: BearerAuth,
    params: web::Path<(Uuid, Uuid)>,
    body: web::Json<PostSubscribeBody>,
) -> crate::Result<impl Responder> {
    let Ok(auth) = auth.token().parse::<Uuid>() else {
        return Ok(Either::Left(("what", StatusCode::NOT_FOUND)));
    };

    let (service_id, group_id) = params.into_inner();
    let body = body.into_inner();

    let count = api_key_scopes::Entity::find()
        .filter(
            api_key_scopes::Column::KeyId
                .eq(auth)
                .and(api_key_scopes::Column::ServiceId.eq(service_id))
                .and(
                    api_key_scopes::Column::GroupId
                        .eq(group_id)
                        .or(api_key_scopes::Column::GroupId.is_null()),
                )
                .and(api_key_scopes::Column::Scope.eq(KeyScope::Sub)),
        )
        .count(&data.db)
        .await?;

    if count == 0 {
        // disguise this
        return Ok(Either::Left(("what", StatusCode::NOT_FOUND)));
    }

    // TODO: encrypt the endpoint and creds

    data.db
        .transaction::<_, (), AnyhowBridge>(move |txn| {
            Box::pin(async move {
                let endpoint = body.endpoint;

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
                            name: ActiveValue::set(body.name),
                            email: ActiveValue::set(body.email),
                            endpoint: ActiveValue::set(endpoint),
                            client_key: ActiveValue::Set(
                                serde_json::to_string(&body.keys).context("json encode keys")?,
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

                group_subscribers::Entity::insert(group_subscribers::ActiveModel {
                    service_id: ActiveValue::Set(service_id),
                    group_id: ActiveValue::Set(group_id),
                    subscriber_id: ActiveValue::set(sub_id),
                })
                .on_conflict_do_nothing()
                .exec(txn)
                .await?;

                Ok(())
            })
        })
        .await?;

    Ok(Either::Right("subscribe someone"))
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
    let Ok(auth) = auth.token().parse::<Uuid>() else {
        return Ok(Either::Left(("what", StatusCode::NOT_FOUND)));
    };

    let (service_id, group_id) = params.into_inner();
    let body = body.into_inner();

    let count = api_key_scopes::Entity::find()
        .filter(
            api_key_scopes::Column::KeyId
                .eq(auth)
                .and(api_key_scopes::Column::ServiceId.eq(service_id))
                .and(
                    api_key_scopes::Column::GroupId
                        .eq(group_id)
                        .or(api_key_scopes::Column::GroupId.is_null()),
                )
                .and(api_key_scopes::Column::Scope.eq(KeyScope::Notify)),
        )
        .count(&data.db)
        .await?;

    if count == 0 {
        // disguise this
        return Ok(Either::Left(("what", StatusCode::NOT_FOUND)));
    }

    let svc = services::Entity::find_by_id(service_id)
        .one(&data.db)
        .await
        .context("get service by id")?
        .context("no service by id")?;

    let mut it = group_subscribers::Entity::find()
        .filter(
            group_subscribers::Column::ServiceId
                .eq(service_id)
                .and(group_subscribers::Column::GroupId.eq(group_id)),
        )
        .find_also_related(subscribers::Entity)
        .paginate(&data.db, 1_000);

    let content_bytes = body.payload.as_bytes();
    let client = IsahcWebPushClient::new().context("construct push client")?;

    while let Some(ch) = it
        .fetch_and_next()
        .await
        .context("next page of subscribers")?
    {
        for sub in ch {
            let (_, Some(sub)) = sub else {
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
            // TODO: set last_notified

            if let Err(_) = client
                .send(builder.build().context("build push message")?)
                .await
            {
                // unsub
            }
        }
    }

    Ok(Either::Right("ok"))
}
