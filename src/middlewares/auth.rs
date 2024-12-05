use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

pub async fn ensure_auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // First of all, ensure that the request has a valid JWT token
    // If not, return an error response
    println!("Auth middleware");

    println!("Request headers: {:?}", req);

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
