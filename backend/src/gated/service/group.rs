use crate::ExtractedAppData;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Either, HttpResponse, Responder};
use anyhow::Context;
use entity::groups;
use sea_orm::prelude::DateTime;
use sea_orm::QueryFilter;
use sea_orm::{ActiveValue, ColumnTrait};
use sea_orm::{EntityTrait, SqlErr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
struct ReturnedGroup {
    group_id: Uuid,
    service_id: Uuid,
    name: String,
    #[serde(with = "crate::util::naive_utc_rfc3339_opt")]
    last_notified: Option<DateTime>,
}

impl Into<ReturnedGroup> for groups::Model {
    fn into(self) -> ReturnedGroup {
        ReturnedGroup {
            group_id: self.group_id,
            service_id: self.service_id,
            name: self.name,
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

#[derive(Deserialize, Debug)]
pub struct PostGroupQuery {
    name: String,
}

#[post("/group")]
async fn post_group(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
    query: web::Query<PostGroupQuery>,
) -> crate::Result<impl Responder> {
    let group_id = Uuid::now_v7();

    let insert_ent = groups::ActiveModel {
        service_id: ActiveValue::set(service_id.into_inner()),
        group_id: ActiveValue::set(group_id),
        name: ActiveValue::set(query.into_inner().name),
        last_notified: Default::default(),
    };

    let returned_ent = match groups::Entity::insert(insert_ent)
        .exec_with_returning(&data.db)
        .await
    {
        Ok(ent) => ent,
        Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            return Ok(Either::Left(("dup name", StatusCode::BAD_REQUEST)));
        }
        Err(e) => return Err(e).context("insert new group")?,
    };

    Ok(Either::Right(web::Json::<ReturnedGroup>(
        returned_ent.into(),
    )))
}

#[get("/group/{group_id}")]
async fn get_one_group(
    data: ExtractedAppData,
    params: web::Path<(Uuid, Uuid)>,
) -> crate::Result<impl Responder> {
    let (service_id, group_id) = params.into_inner();

    let groups = groups::Entity::find()
        .filter(
            groups::Column::ServiceId
                .eq(service_id)
                .and(groups::Column::GroupId.eq(group_id)),
        )
        .all(&data.db)
        .await?;

    Ok(match groups.first() {
        None => Either::Left(HttpResponse::NotFound()),
        Some(group) => Either::Right(web::Json::<ReturnedGroup>(group.clone().into())),
    })
}
