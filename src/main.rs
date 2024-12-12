mod auth;
mod config;
mod contracts;
mod db;
mod errors;
mod middlewares;
mod models;
mod routes;
mod schema;

use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::config::load_env;
use actix_web::{
    middleware::{from_fn, DefaultHeaders, Logger},
    web, App, HttpServer,
};
use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    signers::local::{LocalSigner, PrivateKeySigner},
    sol,
    transports::http::{reqwest::Url, Client, Http},
};
use alloy_node_bindings::Anvil;
use auth::JwtManager;
use contracts::votechain::VotechainContract;
use diesel::PgConnection;
use log::{debug, info};

pub struct AppState {
    jwt_manager: JwtManager,
    connection: Arc<Mutex<PgConnection>>,
    contracts: Contracts,
}

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    VOTECHAIN,
    "contracts/abi/votechain.json"
);

pub type VotechainContractInstance = VOTECHAIN::VOTECHAINInstance<
    Http<Client>,
    FillProvider<
        JoinFill<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider<Http<Client>>,
        Http<Client>,
        Ethereum,
    >,
>;

struct Contracts {
    votechain: VotechainContract,
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

    // Get chain id from config file
    let chain_id = std::env::var("CHAIN_ID").unwrap().parse::<u64>().unwrap();

    // Get RPC endpoint from config file
    let rpc_endpoint = std::env::var("RPC_URL").unwrap();
    // Now, create address from the endpoint
    let rpc_address = Url::from_str(&rpc_endpoint).expect("Invalid RPC endpoint");

    let signer: PrivateKeySigner =
        LocalSigner::from_str(&std::env::var("RELAY_WALLET_PRIVATE_KEY").unwrap())
            .expect("Invalid private key");
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_address);

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

    // Load the contract address from the .env
    let contract_address =
        Address::parse_checksummed(std::env::var("VOTECHAIN_CONTRACT_ADDRESS").unwrap(), None)
            .expect("Invalid contract address. Ensure it is a valid hex string with checksum");

    // get the contract instance
    let contract_instance = VOTECHAIN::new(contract_address, provider);
    // wrap instance in our contract struct to access useful information
    let votechain_contract = contracts::votechain::VotechainContract::new(contract_instance);

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
