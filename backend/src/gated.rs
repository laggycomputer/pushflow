use actix_session::Session;
use actix_web::{get, Responder};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct SessionUser {
    pub user_id: String,
    pub avatar: Option<String>,
}

#[get("/check_auth")]
pub async fn check_auth(session: Session) -> crate::Result<impl Responder> {
    Ok(format!("{:?}", session.entries().to_owned()))
}

#[get("/logout")]
pub async fn logout(session: Session) -> crate::Result<impl Responder> {
    let Ok(Some(_)) = session.get::<SessionUser>("user") else {
        return Ok(("don't think you're logged in but sure", StatusCode::UNAUTHORIZED))
    };

    session.purge();

    Ok(("ok bye", StatusCode::OK))
}