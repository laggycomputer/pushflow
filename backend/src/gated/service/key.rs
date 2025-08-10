use crate::ExtractedAppData;
use actix_web::{get, web, Responder};
use uuid::Uuid;

#[get("")]
async fn get_all_keys(
    data: ExtractedAppData,
    service_id: web::Path<Uuid>,
) -> crate::Result<impl Responder> {
    Ok("get keys")
}
