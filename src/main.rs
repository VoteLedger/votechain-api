mod config;
mod jwt;
mod blockchain;
mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !config::load_env() {
        eprintln!("Failed to load environment variables.");
        std::process::exit(1);
    }

    HttpServer::new(|| {
        App::new()
            .configure(routes::vote_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
