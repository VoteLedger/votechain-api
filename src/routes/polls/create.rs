use actix_web::{post, web, HttpResponse, Responder, Result};
use serde::Serialize;

use crate::{errors::ApiErrorResponse, AppState};

#[derive(Serialize)]
struct DummyPoll {
    question: String,
    answers: Vec<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum PollResponse {
    Success { data: Vec<DummyPoll> },
    _Error { error: String },
}

#[post("/poll")]
pub async fn route(
    form: web::Json<String>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder> {
    let question = form.into_inner();
    let private_key = std::env::var("PRIVATE_KEY").unwrap();

    // FIXME: Finish this
    match app_data
        .contract_service
        .create_poll(&private_key, question)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().body("Poll created successfully")),
        Err(_) => Err(ApiErrorResponse::InternalServerError.into()),
    }
}
