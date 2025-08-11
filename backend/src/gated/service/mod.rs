pub(crate) mod group;
pub(crate) mod key;

use crate::gated::SessionUser;
use crate::util::ReturnedError;
use crate::ExtractedAppData;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{delete, get, patch, post, web, Either, HttpResponse, Responder};
use anyhow::Context;
use entity::{group_subscribers, services, subscribers};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait, SqlErr, TryIntoModel};
use sea_orm::{ActiveValue, ColumnTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
struct ReturnedService {
    service_id: Uuid,
    owner_id: Uuid,
    name: String,
    vapid_public: String,
}

impl From<services::Model> for ReturnedService {
    fn from(val: services::Model) -> Self {
        ReturnedService {
            service_id: val.service_id,
            owner_id: val.owner_id,
            name: val.name,
            vapid_public: val.vapid_public,
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

#[derive(Deserialize)]
struct PatchServiceBody {
    name: Option<String>,
}

#[patch("")]
pub async fn patch_one_service(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
    body: web::Json<PatchServiceBody>,
) -> crate::Result<impl Responder> {
    let service_id = service_id.into_inner();
    let body = body.into_inner();

    let mut service = services::Entity::find_by_id(service_id)
        .one(&data.db)
        .await
        .context("get service to patch")?
        .context("service to patch DNE")?
        .into_active_model();

    if let Some(new_name) = body.name {
        service.name = ActiveValue::Set(new_name);
    }

    match service.clone().update(&data.db).await {
        Ok(_) => {}
        Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            return Ok(Either::Left((
                web::Json::<ReturnedError>("dup name".into()),
                StatusCode::CONFLICT,
            )));
        }
        Err(other) => Err(other).context("update service")?,
    }

    Ok(Either::Right(web::Json::<ReturnedService>(
        service
            .try_into_model()
            .context("service into model")?
            .into(),
    )))
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

#[derive(Serialize)]
struct ReturnedSubscriber {
    service_id: Uuid,
    subscriber_id: Uuid,
    name: Option<String>,
    email: Option<String>,
    groups: Vec<Uuid>,
}

impl ReturnedSubscriber {
    fn new(
        service_id: &Uuid,
        models: (subscribers::Model, Vec<group_subscribers::Model>),
    ) -> ReturnedSubscriber {
        Self {
            service_id: *service_id,
            subscriber_id: models.0.subscriber_id,
            name: models.0.name,
            email: models.0.email,
            groups: models
                .1
                .into_iter()
                .map(|g_sub| g_sub.group_id)
                .collect::<Vec<_>>(),
        }
    }
}

#[get("/subscriber")]
pub async fn get_service_subscriber(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    let service_id = service_id.into_inner();

    let all_subscribers = subscribers::Entity::find()
        .filter(group_subscribers::Column::ServiceId.eq(service_id))
        .find_with_related(group_subscribers::Entity)
        .all(&data.db)
        .await
        .context("get matching subscribers")?;

    Ok(web::Json(
        all_subscribers
            .into_iter()
            .map(|m| ReturnedSubscriber::new(&service_id, m))
            .collect::<Vec<_>>(),
    ))
}

#[delete("/subscriber/{subscriber_id}")]
pub async fn delete_service_subscriber(
    data: ExtractedAppData,
    path: web::Path<(Uuid, Uuid)>,
) -> crate::Result<impl Responder> {
    let (service_id, subscriber_id) = path.into_inner();

    subscribers::Entity::delete_by_id(subscriber_id)
        .exec(&data.db)
        .await
        .context("delete subscriber by id")?;

    Ok("ok deleted")
}

#[derive(Deserialize)]
struct PatchSubscriberBody {
    name: Option<String>,
}

#[patch("/subscriber/{subscriber_id}")]
pub async fn patch_one_subscriber(
    data: ExtractedAppData,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<PatchSubscriberBody>,
) -> crate::Result<impl Responder> {
    let (service_id, subscriber_id) = path.into_inner();
    let body = body.into_inner();

    let mut subscriber = subscribers::Entity::find_by_id(subscriber_id)
        .one(&data.db)
        .await
        .context("get subscriber to patch")?
        .context("subscriber to patch DNE")?
        .into_active_model();

    if let Some(new_name) = body.name {
        subscriber.name = ActiveValue::Set(Some(new_name));
    }

    match subscriber.clone().update(&data.db).await {
        Ok(_) => {}
        Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            return Ok(Either::Left((
                web::Json::<ReturnedError>("dup name".into()),
                StatusCode::CONFLICT,
            )));
        }
        Err(other) => Err(other).context("update subscriber")?,
    }

    Ok(Either::Right("ok"))
}
