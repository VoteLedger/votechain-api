use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web, Error,
};

use crate::AppState;

const UNPROTECTED_PATHS: [&str; 2] = ["/auth/signin", "/health"];

pub async fn ensure_auth(
    data: web::Data<AppState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Skip whole middleware if the path is in the unprotected (public) paths
    if UNPROTECTED_PATHS.contains(&req.path()) {
        return next.call(req).await;
    }

    // Extract the bearer token from the Authorization header
    let token = match req.headers().get("Authorization") {
        Some(t) => t.to_str().unwrap_or(""),
        None => "",
    };

    // Check if the token is empty
    if token.is_empty() {
        return Err(actix_web::error::ErrorUnauthorized(
            "No token provided in Authorization header",
        ));
    }

    // Check if the token is a bearer token
    if !token.starts_with("Bearer ") {
        return Err(actix_web::error::ErrorUnauthorized(
            "Invalid token format. Use Bearer.",
        ));
    }

    // Check if the token is valid
    let token = token.trim_start_matches("Bearer ");
    let result = data.jwt_manager.verify_token(token, false);
    if result.is_err() {
        return Err(actix_web::error::ErrorPreconditionFailed(
            "Invalid or expired token",
        ));
    }

    // FIXME: We need to generate a new token + save it in db
    println!("Token is valid!");

    // continue processing the request
    next.call(req).await
}
