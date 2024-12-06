use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web, Error,
};
use log::{debug, warn};

use crate::{errors::ApiErrorResponse, AppState};

const UNPROTECTED_PATHS: [&str; 2] = ["/auth/signin", "/health"];

pub async fn ensure_auth(
    data: web::Data<AppState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Skip whole middleware if the path is in the unprotected (public) paths
    if UNPROTECTED_PATHS.contains(&req.path()) {
        debug!("Skipping auth middleware for public path: {}", req.path());
        return next.call(req).await;
    }

    // Extract the bearer token from the Authorization header
    let token = match req.headers().get("Authorization") {
        Some(t) => t.to_str().unwrap_or(""),
        None => "",
    };

    // Check if the token is empty
    if token.is_empty() {
        warn!(
            "Received request without token to protected path: {}. Rejecting...",
            req.path()
        );
        return Err(ApiErrorResponse::NoToken.into());
    }

    // Check if the token is a bearer token
    if !token.starts_with("Bearer ") {
        warn!(
            "Received request with invalid token format to protected path: {}. Rejecting...",
            req.path()
        );
        return Err(ApiErrorResponse::MissingBearerToken.into());
    }

    // Check if the token is valid
    let decoded_token = data
        .jwt_manager
        .decode_token(token.trim_start_matches("Bearer "), false);

    if decoded_token.is_err() {
        warn!(
            "Received request with invalid token to protected path: {}. Error: {:?}",
            req.path(),
            decoded_token
        );

        // Return the correct error
        match decoded_token.unwrap_err().kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                // signal user that the token has expired and needs
                // to refresh it
                return Err(ApiErrorResponse::ExpiredToken.into());
            }
            _ => {
                // signal user that the token is invalid
                return Err(ApiErrorResponse::InvalidToken.into());
            }
        };
    }

    // continue processing the request
    next.call(req).await
}
