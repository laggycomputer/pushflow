use crate::{AnyhowBridge, ExtractedAppData};
use actix_web::http::StatusCode;
use actix_web::{post, web, Either, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Context;
use entity::sea_orm_active_enums::KeyScope;
use entity::{api_key_scopes, group_subscribers, subscribers};
use migration::sea_query;
use sea_orm::{ActiveValue, PaginatorTrait};
use sea_orm::{ColumnTrait, TransactionTrait};
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
                }).on_conflict_do_nothing().exec(txn).await?;

                Ok(())
            })
        })
        .await?;

    Ok(Either::Right("subscribe someone"))
}
