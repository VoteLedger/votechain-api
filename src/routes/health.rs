use actix_web::{get, Responder, Result};

#[get("/health")]
async fn route() -> Result<impl Responder> {
    // Check whether signature is already present in database
    Ok("Votechain API is running!")
}
