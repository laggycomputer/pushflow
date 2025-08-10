use crate::ExtractedAppData;
use actix_session::Session;
use actix_web::{get, post, web, Responder};
use anyhow::Context;
use entity::groups;
use sea_orm::prelude::DateTime;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Serialize;
use uuid::Uuid;
use crate::gated::service::ReturnedService;

#[derive(Serialize)]
struct ReturnedGroup {
    group_id: Uuid,
    service_id: Uuid,
    #[serde(with = "crate::util::naive_utc_rfc3339_opt")]
    last_notified: Option<DateTime>,
}

impl Into<ReturnedGroup> for groups::Model {
    fn into(self) -> ReturnedGroup {
        ReturnedGroup {
            group_id: self.group_id,
            service_id: self.service_id,
            last_notified: self.last_notified,
        }
    }
}

#[get("/group")]
async fn get_all_groups(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    let groups = groups::Entity::find()
        .filter(groups::Column::ServiceId.eq(service_id.into_inner()))
        .all(&data.db)
        .await
        .context("fetch services")?;

    Ok(web::Json(
        groups
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<ReturnedGroup>>(),
    ))
}

#[post("/group")]
async fn post_group(
    data: ExtractedAppData,
    session: Session,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    Ok("post group")
}
