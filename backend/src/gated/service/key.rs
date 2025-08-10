use crate::ExtractedAppData;
use actix_web::{Responder, get, web};
use anyhow::Context;
use entity::api_key_scopes;
use entity::api_keys;
use entity::sea_orm_active_enums::KeyScope;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::prelude::DateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct KeyScope2 {
    #[serde(with = "crate::util::active_enum")]
    inner: KeyScope,
}

impl From<KeyScope> for KeyScope2 {
    fn from(val: KeyScope) -> Self {
        KeyScope2 { inner: val }
    }
}

#[derive(Serialize)]
struct ReturnedApiKeyScope {
    pub service_id: Uuid,
    pub group_id: Uuid,
    pub scope: Option<KeyScope2>,
}

impl From<api_key_scopes::Model> for ReturnedApiKeyScope {
    fn from(val: api_key_scopes::Model) -> Self {
        ReturnedApiKeyScope {
            service_id: val.service_id,
            group_id: val.group_id,
            scope: val.scope.map(|s| s.into()),
        }
    }
}

#[derive(Serialize)]
struct ReturnedApiKey {
    service_id: Uuid,
    key_preview: String,
    #[serde(with = "crate::util::naive_utc_rfc3339_opt")]
    last_used: Option<DateTime>,
    scopes: Vec<ReturnedApiKeyScope>,
}

impl From<(api_keys::Model, Vec<api_key_scopes::Model>)> for ReturnedApiKey {
    fn from(val: (api_keys::Model, Vec<api_key_scopes::Model>)) -> Self {
        ReturnedApiKey {
            service_id: val.0.service_id,
            key_preview: val.0.key_id.to_string().split_off(24),
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
            .map(|m| m.into())
            .collect::<Vec<ReturnedApiKey>>(),
    ))
}
