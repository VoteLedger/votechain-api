use actix_web::{web, HttpResponse, Responder};
use crate::jwt;
use crate::blockchain;

#[derive(serde::Deserialize)]
pub struct VoteRequest {
    token: String,
}

pub async fn handle_vote(req_body: web::Json<VoteRequest>) -> impl Responder {
    let jwt_token = &req_body.token;

    match jwt::decode_token(jwt_token) {
        Ok(claims) => {
            if claims.exp < chrono::Utc::now().timestamp() as u64 {
                return HttpResponse::Unauthorized().body("Token expired");
            }

            match blockchain::cast_vote(claims.poll_id, &claims.option, &claims.wallet, &claims.signature).await {
                Ok(_) => HttpResponse::Ok().body("Vote successfully cast!"),
                Err(err) => HttpResponse::InternalServerError().body(format!("Blockchain error: {}", err)),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(format!("Invalid token: {}", err)),
    }
}

pub fn vote_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/vote").route(web::post().to(handle_vote)));
}
