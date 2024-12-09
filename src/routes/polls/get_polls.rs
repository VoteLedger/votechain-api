use actix_web::{get, web, Responder, Result};
use serde::Serialize;

use crate::contracts::votechain::Poll;

#[derive(Serialize)]
#[serde(untagged)]
enum GetPollsResponse {
    Success { polls: Vec<Poll> },
    Error { error: String },
}

#[get("/polls")]
async fn route(app_data: web::Data<crate::AppState>) -> Result<impl Responder> {
    // // Extract contract from app data
    // let contract = &app_data.contracts.votechain;
    //
    // // Fetch all available polls from the blockchain
    // let polls = contract.get_available_polls().await;
    //
    // // Check whether the request was successful
    // if polls.is_err() {
    //     return Ok(web::Json(GetPollsResponse::Error {
    //         error: polls.err().unwrap(),
    //     }));
    // }
    //
    // // Extract polls from the result
    // let polls = polls.unwrap();
    //
    // // Return the polls
    // Ok(web::Json(GetPollsResponse::Success { polls }))

    Ok(web::Json(GetPollsResponse::Success { polls: vec![] }))
}
