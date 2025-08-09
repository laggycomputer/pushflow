use crate::ExtractedAppData;
use crate::gated::SessionUser;
use actix_session::Session;
use actix_web::{Responder, get};
use anyhow::Context;
use entity::services;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

#[get("")]
async fn service(data: ExtractedAppData, session: Session) -> crate::Result<impl Responder> {
    let session_user = session
        .get::<SessionUser>("user")?
        .context("no session user")?;

    dbg!(
        services::Entity::find()
            .filter(services::Column::OwnerId.eq(session_user.user_id))
            .all(&data.db)
            .await?
    );
    Ok("todo")
}
