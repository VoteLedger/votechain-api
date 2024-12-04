mod config;
mod db;
mod routes;

use crate::config::load_env;
use actix_web::{App, HttpServer};
use alloy::providers::{Provider, ProviderBuilder};
use log::{debug, info};

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

    // Link rpc client to alloy
    let rpc_url = std::env::var("RPC_URL").unwrap();

    info!("Linking rpc client to alloy...");
    let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());

    // Ask the chain for its chain id
    let chain_id = provider
        .get_chain_id()
        .await
        .expect("Failed to get chain id");
    info!("Chain ID: {}", chain_id);

    // Start ActiveX web server
    info!("Starting Actix Web server...");
    HttpServer::new(|| {
        App::new()
            .service(crate::routes::signin::route)
            .service(crate::routes::health::route)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
