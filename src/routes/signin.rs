use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SignInResponse {
    message: String,
    token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct SignInRequest {
    message: String,
    signature: String,
    nonce: String,
}

#[post("/auth/signin")]
pub async fn route(web::Json(data): web::Json<SignInRequest>) -> Result<impl Responder> {
    // Extract signature + account from request
    let message = data.message.to_string();
    let signature = data.signature.to_string();
    let nonce = data.nonce.to_string();

    // Now, verify the signature using the public key
    println!("Signature: {}", signature);
    println!("Message: {}", message);
    println!("Nonce: {}", nonce);

    // create response struct
    let response = SignInResponse {
        message: "TESTING GYAT".to_string(),
        token: "nope".to_string(),
        refresh_token: "nope".to_string(),
    };

    // Check whether signature is already present in database
    Ok(web::Json(response))
}
