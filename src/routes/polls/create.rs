use actix_web::{post, web, HttpResponse, Responder, Result};
use alloy::primitives::U256;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
struct CreatePollRequest {
    name: String,
    description: String,
    options: Vec<String>,
    start_time: u64,
    end_time: u64,
}

#[post("/polls")]
pub async fn route(
    request: web::Json<CreatePollRequest>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder> {
    // Extract request data
    let req = request.into_inner();
    let contract = &app_data.contracts.votechain;

    // Call the contract’s create_poll function.
    // Assuming `create_poll` returns a Result<(), ContractError> or similar.
    // Adjust the call to match your actual contract interface.
    // Example:
    let tx_result = contract
        .create_poll(
            req.name,
            req.description,
            req.options,
            U256::from(req.start_time),
            U256::from(req.end_time),
        )
        .await;

    match tx_result {
        Ok(_) => Ok(HttpResponse::Ok().body("Poll created successfully")),
        Err(e) => {
            // Log or handle error as needed
            Ok(HttpResponse::InternalServerError().body(format!("Failed to create poll: {:?}", e)))
        }
    }
}
