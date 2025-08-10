use crate::ExtractedAppData;
use actix_web::{get, post, web, Responder};
use anyhow::Context;
use entity::api_key_scopes;
use entity::api_keys;
use entity::sea_orm_active_enums::KeyScope;
use sea_orm::prelude::DateTime;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct KeyScope2 {
    #[serde(with = "crate::util::active_enum")]
    inner: KeyScope,
}

impl From<KeyScope> for KeyScope2 {
    fn from(val: KeyScope) -> Self {
        KeyScope2 { inner: val }
    }
}

impl From<KeyScope2> for KeyScope {
    fn from(value: KeyScope2) -> Self {
        value.inner
    }
}

#[derive(Serialize)]
struct ReturnedApiKeyScope {
    pub service_id: Uuid,
    pub group_id: Option<Uuid>,
    pub scope: KeyScope2,
}

impl From<api_key_scopes::Model> for ReturnedApiKeyScope {
    fn from(val: api_key_scopes::Model) -> Self {
        ReturnedApiKeyScope {
            service_id: val.service_id,
            group_id: val.group_id,
            scope: val.scope.into(),
        }
    }
}

#[derive(Serialize)]
struct ReturnedApiKey {
    service_id: Uuid,
    name: String,
    key_preview: String,
    #[serde(with = "crate::util::naive_utc_rfc3339_opt")]
    last_used: Option<DateTime>,
    scopes: Vec<ReturnedApiKeyScope>,
}

impl ReturnedApiKey {
    fn new(val: (api_keys::Model, Vec<api_key_scopes::Model>), trunc_key: bool) -> Self {
        let mut key_id = val.0.key_id.to_string();

        ReturnedApiKey {
            service_id: val.0.service_id,
            name: val.0.name,
            key_preview: match trunc_key {
                false => key_id,
                true => key_id.split_off(24)
            },
            last_used: val.0.last_used,
            scopes: val.1.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[get("")]
async fn get_all_keys(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    let groups = api_keys::Entity::find()
        .filter(api_keys::Column::ServiceId.eq(service_id.into_inner()))
        .find_with_related(api_key_scopes::Entity)
        .all(&data.db)
        .await
        .context("fetch keys")?;

    Ok(web::Json(
        groups
            .into_iter()
            .map(|m| ReturnedApiKey::new(m, true))
            .collect::<Vec<_>>(),
    ))
}

#[derive(Deserialize)]
struct SentApiKeyScope {
    group: Option<Uuid>,
    #[serde(with = "crate::util::active_enum")]
    scope: KeyScope,
}

#[derive(Deserialize)]
struct PostApiKeyBody {
    name: String,
    scopes: Vec<SentApiKeyScope>,
}

#[post("")]
async fn post_key(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
    body: web::Json<PostApiKeyBody>,
) -> crate::Result<impl Responder> {
    let service_id = service_id.into_inner();
    let body = body.into_inner();

    let key_id = data
        .db
        .transaction::<_, Uuid, DbErr>(move |txn| {
            Box::pin(async move {
                let key_id = Uuid::new_v4();

                api_keys::ActiveModel {
                    service_id: ActiveValue::set(service_id),
                    key_id: ActiveValue::Set(key_id),
                    name: ActiveValue::Set(body.name),
                    last_used: Default::default(),
                }
                    .insert(txn)
                    .await?;

                // TODO: throw CONFLICT on name unique cons violation

                let scopes = body
                    .scopes
                    .into_iter()
                    .map(|scope| api_key_scopes::ActiveModel {
                        scope_id: ActiveValue::set(Uuid::new_v4()),
                        key_id: ActiveValue::set(key_id),
                        service_id: ActiveValue::set(service_id),
                        group_id: ActiveValue::set(scope.group),
                        scope: ActiveValue::set(scope.scope.into()),
                    })
                    .collect::<Vec<_>>();

                api_key_scopes::Entity::insert_many(scopes)
                    .exec(txn)
                    .await?;

                Ok(key_id)
            })
        })
        .await
        .context("insert key and scopes")?;

    let groups = api_keys::Entity::find_by_id(key_id)
        .find_with_related(api_key_scopes::Entity)
        .all(&data.db)
        .await
        .context("fetch keys to return")?;

    let one_key_and_scopes = groups.into_iter().next().context("should have created one key")?;

    Ok(web::Json(ReturnedApiKey::new(one_key_and_scopes, false)))
}
