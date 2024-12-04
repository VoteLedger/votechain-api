use actix_web::{web, HttpResponse, Responder};
use crate::blockchain;

pub async fn create_poll() -> impl Responder {
    HttpResponse::Ok().body("Poll created")
}

pub async fn end_poll() -> impl Responder {
    HttpResponse::Ok().body("Poll ended")
}

pub fn poll_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/poll")
            .route("/create", web::post().to(create_poll))
            .route("/end", web::post().to(end_poll)),
    );
}
