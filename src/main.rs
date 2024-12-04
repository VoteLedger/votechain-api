mod config;
mod db;
mod routes;

use crate::config::load_env;
use actix_web::{App, HttpServer};
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
