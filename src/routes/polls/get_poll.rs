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
    // // Get VoteChain contract from app_data
    // let contract = &app_data.contracts.votechain;
    // let poll_id = path.into_inner();
    //
    // // Fetch poll with passed ID
    // let poll = contract.get_poll(U256::from(poll_id)).await;

    let poll = Poll {
        name: "Poll 1".to_string(),
        description: "Poll 1".to_string(),
        start_time: U256::from(0),
        end_time: U256::from(0),
        winner: "Winner".to_string(),
        is_ended: true,
    };

    // // Retunr poll if it exists, otherwise return 404
    // match poll {
    //     Ok(e) => {
    //         let poll = e;
    //         Ok(HttpResponse::Ok().json(GetPollApiResponse::Success { poll }))
    //     }
    //     Err(_) => Err(ApiErrorResponse::NotFound.into()),
    // }
    //

    Ok(HttpResponse::Ok().json(GetPollApiResponse::Success { poll }))
}
