mod auth;
mod config;
mod db;
mod middlewares;
mod models;
mod routes;
mod schema;

use std::sync::{Arc, Mutex};

use crate::config::load_env;
use actix_web::{
    middleware::{from_fn, DefaultHeaders},
    web, App, HttpServer,
};
use alloy::providers::{Provider, ProviderBuilder};
use auth::JwtManager;
use diesel::PgConnection;
use log::{debug, info};

pub struct AppState {
    jwt_manager: JwtManager,
    connection: Arc<Mutex<PgConnection>>,
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

    // Create JwtManager to handle JWT stuff
    let jwt_manager = JwtManager::new(
        std::env::var("JWT_SECRET").unwrap(),
        std::env::var("JWT_REFRESH_SECRET").unwrap(),
    );

    // Crete connection with database
    info!("Establishing connection with database...");

    // Establish connection with database + wrap in atomic reference to mutex
    // NOTE: This is needed as we will be sharing the connection across threads
    let connection = Arc::new(Mutex::new(db::establish_connection()));

    // Link rpc client to alloy
    let rpc_url = std::env::var("RPC_URL").unwrap();

    info!("Linking rpc client to Alloy...");
    let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());

    // Ask the chain for its chain id
    let chain_id = provider
        .get_chain_id()
        .await
        .expect("Failed to get chain id");
    info!("Chain ID: {}", chain_id);

    // Build application state
    let app_state = web::Data::new(AppState {
        jwt_manager,
        connection,
    });

    // Start ActiveX web server
    info!("Starting Actix Web server...");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // pass configured jwt manager to all
            .wrap(DefaultHeaders::new().add(("X-Server", "VoteChain-API"))) // add default headers
            .wrap(from_fn(crate::middlewares::auth::ensure_auth))
            .service(crate::routes::health::route) // health route
            .service(crate::routes::auth::signin::route) // auth routes
            .service(crate::routes::auth::refresh::route) // auth routes
            .service(crate::routes::polls::get::route) // polls routes
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
