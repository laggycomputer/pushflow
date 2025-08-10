pub(crate) mod middleware;
pub(crate) mod service;

use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub(crate) struct SessionUser {
    pub user_id: Uuid,
    pub avatar: Option<String>,
}

#[get("/me")]
pub async fn me(session: Session) -> crate::Result<impl Responder> {
    match session.get::<SessionUser>("user")? {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::InternalServerError().body("no user??")),
    }
}

#[post("/logout")]
pub async fn logout(session: Session) -> crate::Result<impl Responder> {
    session.purge();

    Ok(("ok bye", StatusCode::OK))
}
