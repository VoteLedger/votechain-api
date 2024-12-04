use actix_web::{post, web, Responder, Result};
use alloy::signers::{local::PrivateKeySigner, SignerSync};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SignInResponse {
    error: String,
    token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct SignInRequest {
    message: String,
    signature: String,
    account: String,
}

#[post("/auth/signin")]
pub async fn route(web::Json(data): web::Json<SignInRequest>) -> Result<impl Responder> {
    // Extract signature + account from request
    let message = data.message.to_string();
    let recv_sign = data.signature.to_string();
    let address = data.account.to_string();

    // Now, verify the signature using the public key
    println!("Signature: {}", recv_sign);
    println!("Message: {}", message);
    println!("Address: {}", address);

    // Load private key and sign the message
    let signer = PrivateKeySigner::random();
    let signature = signer.sign_message_sync(message.as_bytes());
    if signature.is_err() {
        return Ok(web::Json(SignInResponse {
            error: "Invalid signature".to_string(),
            token: "nope".to_string(),
            refresh_token: "nope".to_string(),
        }));
    }
    let signature = signature.unwrap();

    // Recover address from signature
    let recovered_address = signature.recover_address_from_msg(message);
    if recovered_address.is_err() {
        return Ok(web::Json(SignInResponse {
            error: "Invalid signature".to_string(),
            token: "nope".to_string(),
            refresh_token: "nope".to_string(),
        }));
    }
    let recovered_address = recovered_address.unwrap();

    println!("Passed ddress: {}", address);
    println!("Recovered address: {}", recovered_address);
    println!("Our address: {}", signer.address());

    // Check whether the recovered address is the same as the address in the request
    if address != recovered_address.to_string() {
        return Ok(web::Json(SignInResponse {
            error: "Invalid signature".to_string(),
            token: "nope".to_string(),
            refresh_token: "nope".to_string(),
        }));
    }

    // create response struct
    let response = SignInResponse {
        error: "TESTING GYAT".to_string(),
        token: "nope".to_string(),
        refresh_token: "nope".to_string(),
    };

    // Check whether signature is already present in database
    Ok(web::Json(response))
}
