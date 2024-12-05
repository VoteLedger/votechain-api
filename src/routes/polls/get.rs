use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
enum GetPollsResponse {
    // Success { data: TokenPair },
    // FIXME: As soon as diesel is integrated, use the Poll model struct!
    Success { error: String },
    _Error { error: String },
}

#[get("/polls")]
async fn route() -> Result<impl Responder> {
    // Check whether signature is already present in database
    Ok(web::Json(GetPollsResponse::Success {
        error: "<VoteChain-API>: Not implemented yet! Hi from backend btw".to_string(),
    }))
}
