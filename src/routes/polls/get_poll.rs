use actix_web::{get, web, HttpResponse, Responder, Result};
use alloy::primitives::ruint::aliases::U256;
use serde::Serialize;

use crate::{contracts::votechain::Poll, errors::ApiErrorResponse};

#[derive(Serialize)]
#[serde(untagged)]
enum GetPollApiResponse {
    Success { poll: Poll },
}

#[get("/poll/{id}")]
pub async fn route(
    path: web::Path<u64>,
    app_data: web::Data<crate::AppState>,
) -> Result<impl Responder> {
    // Get VoteChain contract from app_data
    let contract = &app_data.contracts.votechain;
    let poll_id = path.into_inner();

    // Fetch poll with passed ID
    let poll = contract.get_poll(U256::from(poll_id)).await;

    // Retunr poll if it exists, otherwise return 404
    match poll {
        Ok(e) => {
            let poll = e;
            Ok(HttpResponse::Ok().json(GetPollApiResponse::Success { poll }))
        }
        Err(_) => Err(ApiErrorResponse::NotFound.into()),
    }
}
