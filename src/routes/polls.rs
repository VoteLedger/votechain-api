//pub mod get;

use actix_web::{get, post, web, HttpResponse};
use crate::services::contract_service::ContractService;

#[get("/poll/{id}")]
async fn get_poll(
    path: web::Path<u64>,
    contract_service: web::Data<ContractService>,
) -> HttpResponse {
    match contract_service.get_poll(path.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/poll")]
async fn create_poll(
    form: web::Json<String>,
    contract_service: web::Data<ContractService>,
) -> HttpResponse {
    let question = form.into_inner();
    let private_key = std::env::var("PRIVATE_KEY").unwrap();
    match contract_service.create_poll(&private_key, question).await {
        Ok(_) => HttpResponse::Ok().body("Poll created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
