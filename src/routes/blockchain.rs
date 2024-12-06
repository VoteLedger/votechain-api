use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;

#[get("/blockchain/get_data")]
async fn get_data(app_state: web::Data<AppState>) -> impl Responder {
    let blockchain_manager = &app_state.blockchain_manager;

    match blockchain_manager.call_get_data().await {
        Ok(data) => HttpResponse::Ok().body(format!("Data from contract: {}", data)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
