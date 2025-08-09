use crate::ExtractedAppData;
use actix_session::Session;
use actix_web::{get, Responder};

#[get("/")]
async fn service(data: ExtractedAppData, session: Session) -> crate::Result<impl Responder> {


    Ok("todo")
}