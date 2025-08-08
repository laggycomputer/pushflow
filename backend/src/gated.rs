use actix_session::Session;
use actix_web::{get, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct SessionUser {
    pub user_id: String,
    pub avatar: Option<String>,
}

#[get("/check_auth")]
pub async fn check_auth(session: actix_web::web::Data<Session>) -> crate::Result<impl Responder> {
    Ok(format!("{:?}", session.entries().to_owned()))
}
