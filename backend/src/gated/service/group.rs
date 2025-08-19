use crate::ExtractedAppData;
use crate::util::ReturnedError;
use actix_web::http::StatusCode;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{Either, HttpRequest, HttpResponse, Responder, delete, get, patch, post, web};
use anyhow::{Context, anyhow};
use entity::groups;
use sea_orm::prelude::DateTime;
use sea_orm::{ActiveModelTrait, IntoActiveModel, QueryFilter, TryIntoModel};
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

impl From<groups::Model> for ReturnedGroup {
    fn from(val: groups::Model) -> Self {
        ReturnedGroup {
            group_id: val.group_id,
            service_id: val.service_id,
            name: val.name,
            last_notified: val.last_notified,
        }
    }
}

#[get("")]
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
pub struct PostGroupBody {
    name: String,
}

#[post("")]
async fn post_group(
    data: ExtractedAppData,
    req: HttpRequest,
    service_id: web::Path<Uuid>,
    body: web::Json<PostGroupBody>,
) -> crate::Result<impl Responder> {
    let group_id = Uuid::now_v7();

    let insert_ent = groups::ActiveModel {
        service_id: ActiveValue::set(service_id.into_inner()),
        group_id: ActiveValue::set(group_id),
        name: ActiveValue::set(body.into_inner().name),
        last_notified: Default::default(),
    };

    let returned_ent = match groups::Entity::insert(insert_ent)
        .exec_with_returning(&data.db)
        .await
    {
        Ok(ent) => ent,
        Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            return Ok(Either::Left((
                web::Json::<ReturnedError>("dup name".into()),
                StatusCode::CONFLICT,
            )));
        }
        Err(e) => return Err(e).context("insert new group")?,
    };

    let mut url = req.full_url();

    url.path_segments_mut()
        .map_err(|_| anyhow!("service POST isn't base"))?
        .push(&returned_ent.group_id.to_string());

    let mut res = web::Json::<ReturnedGroup>(returned_ent.into()).respond_to(&req);

    let _ = std::mem::replace(res.status_mut(), StatusCode::CREATED);
    res.headers_mut().insert(
        HeaderName::from_static("location"),
        HeaderValue::from_str(url.as_ref()).context("Location value as HeaderValue")?,
    );

    Ok(Either::Right(res))
}

#[get("/{group_id}")]
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

#[derive(Deserialize)]
struct PatchGroupBody {
    name: Option<String>,
}

#[patch("/{group_id}")]
pub async fn patch_one_group(
    data: ExtractedAppData,
    params: web::Path<(Uuid, Uuid)>,
    body: web::Json<PatchGroupBody>,
) -> crate::Result<impl Responder> {
    let (service_id, group_id) = params.into_inner();
    let body = body.into_inner();

    let mut group = groups::Entity::find_by_id((service_id, group_id))
        .one(&data.db)
        .await
        .context("get group to patch")?
        .context("group to patch DNE")?
        .into_active_model();

    if let Some(new_name) = body.name {
        group.name = ActiveValue::Set(new_name);
    }

    match group.clone().update(&data.db).await {
        Ok(_) => {}
        Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            return Ok(Either::Left((
                web::Json::<ReturnedError>("dup name".into()),
                StatusCode::CONFLICT,
            )));
        }
        Err(other) => Err(other).context("update group")?,
    }

    Ok(Either::Right(web::Json::<ReturnedGroup>(
        group.try_into_model().context("group into model")?.into(),
    )))
}

#[delete("/{group_id}")]
pub async fn delete_one_group(
    data: ExtractedAppData,
    params: web::Path<(Uuid, Uuid)>,
) -> crate::Result<impl Responder> {
    let (service_id, group_id) = params.into_inner();

    groups::Entity::delete_by_id((service_id, group_id))
        .exec(&data.db)
        .await
        .context("delete group by id")?;

    Ok("crab")
}
