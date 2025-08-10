use crate::ExtractedAppData;
use actix_web::{get, web, Responder};
use anyhow::Context;
use entity::api_key_scopes;
use entity::api_keys;
use entity::sea_orm_active_enums::KeyScope;
use sea_orm::prelude::DateTime;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveEnum, ColumnTrait};
use serde::{Serialize, Serializer};
use uuid::Uuid;

#[derive(Serialize)]
struct KeyScope2 {
    #[serde(with = "crate::util::active_enum")]
    inner: KeyScope,
}

impl Into<KeyScope2> for KeyScope {
    fn into(self) -> KeyScope2 {
        KeyScope2 { inner: self }
    }
}

#[derive(Serialize)]
struct ReturnedApiKeyScope {
    pub service_id: Uuid,
    pub group_id: Uuid,
    pub scope: Option<KeyScope2>,
}

impl Into<ReturnedApiKeyScope> for api_key_scopes::Model {
    fn into(self) -> ReturnedApiKeyScope {
        ReturnedApiKeyScope {
            service_id: self.service_id,
            group_id: self.group_id,
            scope: self.scope.map(|s| s.into()),
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

impl Into<ReturnedApiKey> for (api_keys::Model, Vec<api_key_scopes::Model>) {
    fn into(self) -> ReturnedApiKey {
        ReturnedApiKey {
            service_id: self.0.service_id,
            key_preview: self.0.key_id.to_string().split_off(24),
            last_used: self.0.last_used,
            scopes: self.1.into_iter().map(|x| x.into()).collect(),
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
