use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

const UNPROTECTED_PATHS: [&str; 2] = ["/auth/signin", "/health"];

pub async fn ensure_auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Skip whole middleware if the path is in the unprotected (public) paths
    if UNPROTECTED_PATHS.contains(&req.path()) {
        return next.call(req).await;
    }

    // First of all, ensure that the request has a valid JWT token
    // If not, return an error response
    println!("Auth middleware called from: {}", req.path());

    // Extract cookie from request
    let cookie = req.cookie("token");
    if cookie.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("No token provided"));
    }

    // Extract the token from the cookie
    let token = cookie.unwrap();

    // Ensure that the token is valid
    println!("Cookie value: {}", token.value());

    // continue processing the request
    next.call(req).await
}
