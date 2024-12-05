use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::AppState;

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
    Error { error: String },
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
    let claims = shared_data.jwt_manager.verify_token(&refresh_token, true);
    if claims.is_err() {
        return Ok(web::Json(RefreshResponse::Error {
            error: "Invalid token".to_string(),
        }));
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
