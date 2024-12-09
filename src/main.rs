mod auth;
mod config;
mod contracts;
mod db;
mod errors;
mod middlewares;
mod models;
mod routes;
mod schema;

use std::sync::{Arc, Mutex};

use crate::config::load_env;
use actix_web::{
    middleware::{from_fn, DefaultHeaders, Logger},
    web, App, HttpServer,
};
use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use auth::JwtManager;
use diesel::PgConnection;
use log::{debug, info};

struct Contracts {
    votechain: contracts::votechain::VotechainContract,
}

pub struct AppState {
    jwt_manager: JwtManager,
    connection: Arc<Mutex<PgConnection>>,
    contracts: Contracts,
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

    // create link with available contracts + connect to blockchain
    let rpc_url = Url::parse(&std::env::var("RPC_URL").unwrap()).unwrap();

    // Get chain id from config file
    let chain_id = std::env::var("CHAIN_ID").unwrap().parse::<u64>().unwrap();

    // Get wallet information from the config file
    let srv_wallet_key = std::env::var("RELAY_WALLET_PRIVATE_KEY").unwrap();

    // Create signer based on private key
    let signer: PrivateKeySigner = srv_wallet_key.parse().expect("Invalid private key");
    let wallet = EthereumWallet::new(signer);

    // create provider + link the wallet for signing
    info!("Linking rpc client to Alloy...");
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);

    // Ask the chain for its chain id
    let actual_chain_id = provider
        .get_chain_id()
        .await
        .expect("Failed to get chain id");
    info!("Connection successful. Got chain id: {}", actual_chain_id);

    // Ensure that the chain Id matches the one in the config
    if actual_chain_id != chain_id {
        panic!(
            "Chain ID mismatch. Expected: {}, Got: {}",
            chain_id, actual_chain_id
        );
    } else {
        info!("Chain ID matches the one in the config file. Proceeding...");
    }

    // Now, build the VoteChain contract abstraction
    let address =
        Address::parse_checksummed(std::env::var("VOTECHAIN_CONTRACT_ADDRESS").unwrap(), None)
            .expect("Invalid contract address. Ensure it is a valid hex string with checksum");
    let votechain_contract =
        contracts::votechain::VotechainContract::new(address, provider.root().clone());

    // Build application state
    let app_state = web::Data::new(AppState {
        jwt_manager,
        connection,
        contracts: Contracts {
            votechain: votechain_contract,
        },
    });

    // Start ActiveX web server
    info!("Starting Actix Web server...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone()) // pass state to entire application
            .wrap(DefaultHeaders::new().add(("X-Server", "VoteChain-API"))) // add default headers
            .wrap(from_fn(crate::middlewares::auth::ensure_auth))
            .service(crate::routes::health::route) // health route
            .service(crate::routes::auth::signin::route) // sign up / login route
            .service(crate::routes::auth::refresh::route) // token refresh route
            .service(crate::routes::polls::get_poll::route) // Route to get a poll
            .service(crate::routes::polls::get_polls::route) // Route to get all available polls
            .service(crate::routes::polls::create::route) // Route to create a poll in the contract
            .service(crate::routes::polls::cast_vote::route)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
