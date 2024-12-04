mod config;
mod db;

use crate::config::load_env;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SignInRequest {
    message: String,
    signature: String,
    nonce: String,
}

#[derive(Serialize)]
struct SignInResponse {
    message: String,
    token: String,
    refresh_token: String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/auth/signin")]
async fn signin(web::Json(data): web::Json<SignInRequest>) -> Result<impl Responder> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from dotenv file
    info!("Loading environment variables");
    let status = load_env();
    if !status {
        print!("Unable to load environment variables. Check your logs.");
        std::process::exit(1);
    }

    // initialize logger
    env_logger::init();
    debug!("Logger initialized!");

    // Crete connection with database
    info!("Establishing connection with database...");
    let _connection = db::establish_connection();

    // Start ActiveX web server
    info!("Starting Actix Web server...");
    HttpServer::new(|| App::new().service(signin))
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
}
