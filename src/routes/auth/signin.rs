use std::str::FromStr;

use actix_web::{post, web, Responder, Result};
use alloy::primitives::PrimitiveSignature;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{errors::ApiErrorResponse, models::users::User, AppState};

#[derive(Serialize)]
pub struct TokenPair {
    pub token: String,         // Access token
    pub refresh_token: String, // Refresh token
}

pub struct Identity {
    pub address: String, // Address of the user
}

#[derive(Serialize)]
#[serde(untagged)]
enum SignInResponse {
    Success { data: TokenPair },
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
    app_state: web::Data<AppState>,
    web::Json(data): web::Json<SignInRequest>,
) -> Result<impl Responder> {
    // Extract signature + account from request
    let message = data.message.to_string();
    let recv_sign = data.signature.to_string();
    let address = data.account.to_string();

    // Load signature from string
    let signature = PrimitiveSignature::from_str(&recv_sign);

    if signature.is_err() {
        return Err(ApiErrorResponse::InvalidSignature.into());
    }
    let signature = signature.unwrap();

    // Now, recover the address from the signature
    let recovered_address = signature.recover_address_from_msg(message);
    if recovered_address.is_err() {
        return Err(ApiErrorResponse::InvalidSignature.into());
    }

    // Check whether the recovered address is the same as the address in the request
    // NOTE: Ethereum addresses are case-insensit
    // (https://ethereum.stackexchange.com/questions/2045/is-ethereum-wallet-address-case-sensitive)
    if address.to_lowercase() != recovered_address.unwrap().to_string().to_lowercase() {
        return Err(ApiErrorResponse::InvalidSignature.into());
    }

    //
    // Generate a new pair of refresh and access tokens
    //

    let identity = Identity {
        address: address.clone(),
    };

    // Generate JWT token for the user!
    let token_pair = app_state.jwt_manager.generate_token_pair(identity);

    //
    // Persist data in the database
    //

    // Extract connection from shared_data + lock mutex
    let mut connection = app_state
        .connection
        .lock()
        .expect("Error getting connection");

    // Pass mutable reference of connection to get_user_by_address
    let result = User::get_user_by_address(&mut connection, &address);

    // Get current timestamp
    let now = std::time::SystemTime::now();

    // If result is not found, we need to create a new user
    if result.is_err() {
        // Create new user
        let new_user = User {
            primary_account: address.clone(),
            refresh_token: token_pair.refresh_token.clone(),
            last_login: Some(now),
            created_at: Some(now),
        };

        // Save user to database
        let result = new_user.save(&mut connection);
        if result.is_err() {
            error!("Unable create new user: {} in VoteChain", address);

            // Get the error and print some details
            error!("Error: {}", result.unwrap_err());

            // Return error response
            return Err(actix_web::error::ErrorInternalServerError(
                "Unable to record your new VoteChain account. Try again later.",
            ));
        }

        // Log user creation
        info!("User created: {}", address);
    } else {
        // Fetch current user from result
        let mut user = result.unwrap();

        // Update refresh token + last login
        user.refresh_token = token_pair.refresh_token.clone();
        user.last_login = Some(std::time::SystemTime::now());

        // Save updated user to database
        let result = user.update(&mut connection);
        if result.is_err() {
            error!(
                "Unable to record refresh token for user: {} in VoteChain",
                address
            );

            // Get the error and print some details
            error!("Error: {}", result.unwrap_err());

            // Return error response
            return Err(ApiErrorResponse::InternalServerError.into());
        }
    }

    // Check whether signature is already present in database
    Ok(web::Json(SignInResponse::Success { data: token_pair }))
}
