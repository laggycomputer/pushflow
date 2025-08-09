use crate::ExtractedAppData;
use actix_session::Session;
use actix_web::{post, web, Responder};
use uuid::Uuid;

#[post("/group")]
async fn post_group(
    data: ExtractedAppData,
    session: Session,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    Ok("todo")
}
