use actix_web::{get, web, HttpResponse, Responder, Result};

use crate::errors::ApiErrorResponse;

#[get("/poll/{id}")]
pub async fn route(
    path: web::Path<u64>,
    shared_data: web::Data<crate::AppState>,
) -> Result<impl Responder> {
    // match shared_data
    //     .contract_service
    //     .get_poll(path.into_inner())
    //     .await
    // {
    //     Ok(result) => Ok(HttpResponse::Ok().json(result)),
    //     Err(_) => Err(ApiErrorResponse::InternalServerError.into()),
    // }
    Ok(HttpResponse::Ok())
}
