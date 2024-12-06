use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::{errors::ApiErrorResponse, models::users::User, AppState};

#[derive(Serialize, Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Serialize)]
struct FreshToken {
    token: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum RefreshResponse {
    Success { data: FreshToken },
}

// Create secret pair struct to be used later!
#[post("/auth/refresh")]
pub async fn route(
    shared_data: web::Data<AppState>,
    web::Json(data): web::Json<RefreshRequest>,
) -> Result<impl Responder> {
    // Get refresh token from the request
    let refresh_token = data.refresh_token.to_string();

    // Now, decode the refresh token using the auth util module
    //  - If the token is invalid, return an error response
    //  - If the token is valid, generate a new access token with the session data
    //    saved in the refresh token
    let decoded_token = shared_data.jwt_manager.decode_token(&refresh_token, true);

    // If the refresh token is invalid, just reject the request
    if decoded_token.is_err() {
        return Err(ApiErrorResponse::InvalidToken.into());
    }

    // Extract account address from token claims
    let address = decoded_token.unwrap().claims.sub;

    // If its valid, we need to ensure that it is actually linked to the user in the DB
    // NOTE: This allows to revoke the refresh token if the user logs out!
    let mut connection = shared_data.connection.lock().unwrap();
    let user = User::get_user_by_address(&mut connection, &address);
    if user.is_err() {
        return Err(ApiErrorResponse::InvalidToken.into());
    }

    // We now check whether the refresh token is still linked to the account, or it has been
    // replaced
    if user.unwrap().refresh_token != refresh_token {
        return Err(ApiErrorResponse::TokenMismatch.into());
    }

    // If the token is valid, generate new one!
    // FIXME: This must be completed!

    // Respond with a dummy token
    Ok(web::Json(RefreshResponse::Success {
        data: FreshToken {
            token: "dummy_token".to_string(),
        },
    }))
}
