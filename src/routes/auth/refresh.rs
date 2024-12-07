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
    app_state: web::Data<AppState>,
    web::Json(data): web::Json<RefreshRequest>,
) -> Result<impl Responder> {
    // Get refresh token from the request
    let refresh_token = data.refresh_token.to_string();

    // Now, decode the refresh token using the auth util module
    //  - If the token is invalid, return an error response
    //  - If the token is valid, generate a new access token with the session data
    //    saved in the refresh token
    let decoded_token = app_state.jwt_manager.decode_token(&refresh_token, true);

    // If the token is invalid (expired is okay at the moment) return an error
    if decoded_token.is_err()
        && !jsonwebtoken::errors::ErrorKind::ExpiredSignature
            .eq(decoded_token.as_ref().unwrap_err().kind())
    {
        return Err(ApiErrorResponse::InvalidToken.into());
    }

    // Extract account address from token claims
    let address = &decoded_token.as_ref().unwrap().claims.sub;

    // If its valid, we need to ensure that it is actually linked to the user in the DB
    // NOTE: This allows to revoke the refresh token if the user logs out!
    let mut connection = app_state.connection.lock().unwrap();
    let user = User::get_user_by_address(&mut connection, address);
    if user.is_err() {
        return Err(ApiErrorResponse::InvalidToken.into());
    }

    // We now check whether the refresh token is still linked to the account, or it has been
    // replaced
    let user = user.unwrap();
    if user.refresh_token != refresh_token {
        return Err(ApiErrorResponse::TokenMismatch.into());
    }

    // First of all, we need to verify that the refresh token is still valid
    if app_state
        .jwt_manager
        .decode_token(&refresh_token, true)
        .is_err()
    {
        // clean the database of the refresh token
        user.revoke_refresh_token(&mut connection);
        // return an error response
        return Err(ApiErrorResponse::ExpiredToken.into());
    }

    // If the token is invalid (expired is okay at the moment) return an error
    if decoded_token.is_err()
        && jsonwebtoken::errors::ErrorKind::ExpiredSignature
            .eq(decoded_token.as_ref().unwrap_err().kind())
    {
        // clean the database of the refresh token
        user.revoke_refresh_token(&mut connection);
        // return an error response
        return Err(ApiErrorResponse::ExpiredToken.into());
    }

    // We need to check again for errors: if the token is expired, we need to clean the db record

    // Generate new access token from the refresh token
    let new_access_token = app_state
        .jwt_manager
        .new_access_token_from_refresh(&refresh_token);

    if new_access_token.is_none() {
        return Err(ApiErrorResponse::InvalidToken.into());
    }

    // Respond with a dummy token
    Ok(web::Json(RefreshResponse::Success {
        data: FreshToken {
            token: new_access_token.unwrap(),
        },
    }))
}
