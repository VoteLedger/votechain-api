use std::str::FromStr;

use crate::auth::{generate_jwt_token_pair, Identity, SecretPair};
use actix_web::{post, web, Responder, Result};
use alloy::primitives::PrimitiveSignature;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
enum SignInResponse {
    Success {
        token: String,
        refresh_token: String,
    },
    Error {
        error: String,
    },
}

#[derive(Deserialize)]
struct SignInRequest {
    message: String,
    signature: String,
    account: String,
}

// Create secret pair struct to be used later!
#[post("/auth/signin")]
pub async fn route(web::Json(data): web::Json<SignInRequest>) -> Result<impl Responder> {
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
    // NOTE: Ethereum addresses are case-insensit// NOTE: Ethereum addresses are case-insensitive
    // (https://ethereum.stackexchange.com/questions/2045/is-ethereum-wallet-address-case-sensitive)
    if address.to_lowercase() != recovered_address.to_string().to_lowercase() {
        return Ok(web::Json(SignInResponse::Error {
            error: "Invalid signature. Address mismatch.".to_string(),
        }));
    }

    // Build identity struct
    let identity = Identity { address };

    // load secrets from environment variables
    let secrets = SecretPair {
        secret: std::env::var("JWT_SECRET").unwrap(),
        refresh_secret: std::env::var("JWT_REFRESH_SECRET").unwrap(),
    };

    // Generate JWT token for the user!
    let token_pair = generate_jwt_token_pair(identity, secrets);

    info!("\tAccess token: {}", token_pair.token);
    info!("\tRefresh token: {}", token_pair.refresh_token);

    // Check whether signature is already present in database
    Ok(web::Json(SignInResponse::Success {
        token: token_pair.token,
        refresh_token: token_pair.refresh_token,
    }))
}
