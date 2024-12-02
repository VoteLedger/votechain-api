mod config;

use crate::config::load_env;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from dotenv file
    let status = load_env();
    if !status {
        print!("Unable to load environment variables. Check your logs.");
        std::process::exit(1);
    }

    // Start ActiveX web server
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
}
