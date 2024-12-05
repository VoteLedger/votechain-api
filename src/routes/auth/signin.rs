use std::str::FromStr;

use actix_web::{post, web, Responder, Result};
use alloy::primitives::PrimitiveSignature;
use log::info;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize)]
pub struct TokenPair {
    pub token: String,         // Access token
    pub refresh_token: String, // Refresh token
}

pub struct Identity {
    pub address: String, // User's blockchain address
}

#[derive(Serialize)]
#[serde(untagged)]
enum SignInResponse {
    Success { data: TokenPair },
    Error { error: String },
}

#[derive(Deserialize)]
struct SignInRequest {
    message: String,
    signature: String,
    account: String,
}

// Create secret pair struct to be used later!
#[post("/auth/signin")]
pub async fn route(
    shared_data: web::Data<AppState>,
    web::Json(data): web::Json<SignInRequest>,
) -> Result<impl Responder> {
    // Extract signature + account from request
    let message = data.message.to_string();
    let recv_sign = data.signature.to_string();
    let address = data.account.to_string();

    // Load signature from string
    let signature = PrimitiveSignature::from_str(&recv_sign);

    if signature.is_err() {
        return Ok(web::Json(SignInResponse::Error {
            error: "Invalid signature".to_string(),
        }));
    }
    let signature = signature.unwrap();

    // Now, recover the address from the signature
    let recovered_address = signature.recover_address_from_msg(message);
    if recovered_address.is_err() {
        return Ok(web::Json(SignInResponse::Error {
            error: "Invalid signature".to_string(),
        }));
    }
    let recovered_address = recovered_address.unwrap();

    // Check whether the recovered address is the same as the address in the request
    // NOTE: Ethereum addresses are case-insensit
    // (https://ethereum.stackexchange.com/questions/2045/is-ethereum-wallet-address-case-sensitive)
    if address.to_lowercase() != recovered_address.to_string().to_lowercase() {
        return Ok(web::Json(SignInResponse::Error {
            error: "Invalid signature. Address mismatch.".to_string(),
        }));
    }

    // Build identity struct
    let identity = Identity { address };

    // Generate JWT token for the user!
    let token_pair = shared_data.jwt_manager.generate_token_pair(identity);

    info!("\tAccess token: {}", token_pair.token);
    info!("\tRefresh token: {}", token_pair.refresh_token);

    // Check whether signature is already present in database
    Ok(web::Json(SignInResponse::Success { data: token_pair }))
}
