use actix_web::{cookie::time::OffsetDateTime, post, web, HttpResponse, Responder, Result};
use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
struct CreatePollRequest {
    name: String,
    description: String,
    options: Vec<String>,
    start_time: U256,
    end_time: U256,
}

#[derive(Serialize)]
struct CreatePollResponse {
    poll_id: U256,
}

#[post("/polls")]
pub async fn route(
    request: web::Json<CreatePollRequest>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder> {
    // Extract request data
    let req = request.into_inner();
    let contract = &app_data.contracts.votechain;

    // Convert the start and end times to the number of seconds since the Unix epoch.
    // NOTE: This is necessary because the OffsetDateTime type is not directly compatible with U256.
    // let unix_start_time = req.start_time.unix_timestamp();
    // let unix_end_time = req.end_time.unix_timestamp();

    // Call the contract’s create_poll function.
    // Assuming `create_poll` returns a Result<(), ContractError> or similar.
    // Adjust the call to match your actual contract interface.
    // Example:
    let tx_result = contract
        .create_poll(
            req.name,
            req.description,
            req.options,
            req.start_time,
            req.end_time,
        )
        .await;

    match tx_result {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            // Log or handle error as needed
            Ok(HttpResponse::InternalServerError().body(format!("Failed to create poll: {:?}", e)))
        }
    }
}
