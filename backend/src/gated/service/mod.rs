pub(crate) mod group;
pub(crate) mod key;

use crate::gated::SessionUser;
use crate::ExtractedAppData;
use actix_session::Session;
use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use anyhow::Context;
use entity::services;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
struct ReturnedService {
    service_id: Uuid,
    owner_id: Uuid,
    name: String,
}

impl From<services::Model> for ReturnedService {
    fn from(val: services::Model) -> Self {
        ReturnedService {
            service_id: val.service_id,
            owner_id: val.owner_id,
            name: val.name,
        }
    }
}

#[get("")]
async fn get_service(data: ExtractedAppData, session: Session) -> crate::Result<impl Responder> {
    let session_user = session
        .get::<SessionUser>("user")?
        .context("no session user")?;

    let owned_services = services::Entity::find()
        .filter(services::Column::OwnerId.eq(session_user.user_id))
        .all(&data.db)
        .await?;

    Ok(web::Json(
        owned_services
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<ReturnedService>>(),
    ))
}

#[derive(Deserialize, Debug)]
pub struct PostServiceBody {
    name: String,
}

#[post("")]
async fn post_service(
    data: ExtractedAppData,
    session: Session,
    body: web::Json<PostServiceBody>,
) -> crate::Result<impl Responder> {
    let session_user = session
        .get::<SessionUser>("user")?
        .context("no session user")?;

    let vapid = vapid::Key::generate().context("gen new vapid pair")?;

    let service_id = Uuid::now_v7();

    let insert_ent = services::ActiveModel {
        service_id: sea_orm::Set(service_id),
        owner_id: sea_orm::Set(session_user.user_id),
        name: sea_orm::Set(body.name.clone()),
        vapid_public: sea_orm::Set(vapid.to_public_raw()),
        vapid_private: sea_orm::Set(vapid.to_private_raw()),
    };

    let returned_ent = services::Entity::insert(insert_ent)
        .exec_with_returning(&data.db)
        .await
        .context("insert new service")?;

    Ok(web::Json::<ReturnedService>(returned_ent.into()))
}

#[get("")]
pub async fn get_one_service(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    let service_by_id_and_owned = services::Entity::find()
        .filter(services::Column::ServiceId.eq(service_id.into_inner()))
        .all(&data.db)
        .await?;

    Ok(match service_by_id_and_owned.first() {
        None => Either::Left(HttpResponse::NotFound()),
        Some(service) => Either::Right(web::Json::<ReturnedService>(service.clone().into())),
    })
}

#[delete("")]
pub async fn delete_one_service(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    services::Entity::delete_by_id(service_id.into_inner())
        .exec(&data.db)
        .await
        .context("delete service by id")?;

    Ok("crab")
}
