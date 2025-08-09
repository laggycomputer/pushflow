use crate::ExtractedAppData;
use actix_session::Session;
use actix_web::{Responder, post, web};
use uuid::Uuid;

#[post("/group")]
async fn post_group(
    data: ExtractedAppData,
    session: Session,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    Ok("todo")
}
