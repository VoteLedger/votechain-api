use actix_web::{post, web, HttpResponse, Responder, Result};
use alloy::primitives::ruint::aliases::U256;
use serde::Deserialize;

use crate::errors::ApiErrorResponse;

// Definisci la struttura per i dati di input della richiesta POST
#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub poll_id: u64,   // ID del sondaggio
    pub option: String, // Opzione selezionata dall'utente
}

#[post("/poll/{id}")]
pub async fn route(
    path: web::Path<u64>,                 // Ottieni il poll_id dalla path
    data: web::Json<CastVoteRequest>,     // Dati inviati dall'utente come JSON
    app_data: web::Data<crate::AppState>, // Stato dell'applicazione con i contratti
) -> Result<impl Responder> {
    // let poll_id_path = path.into_inner();   // Estrarre poll_id dalla path
    // let poll_id_body = data.poll_id;       // Estrarre poll_id dal corpo JSON
    //
    // // Confronta gli ID per sicurezza
    // if poll_id_path != poll_id_body {
    //     return Err(ApiErrorResponse::NotFound.into());
    // }
    //
    // let option = data.option.clone(); // Ottieni l'opzione dal corpo JSON
    //
    // // Ottieni il contratto del sondaggio
    // let contract = &app_data.contracts.votechain;
    //
    // // Effettua la chiamata alla funzione cast_vote del contratto
    // let result = contract
    //     .cast_vote(
    //         U256::from(poll_id_path),
    //         option.clone())
    //     .await;
    //
    // // Gestisci il risultato della chiamata al contratto
    // match result {
    //     Ok(_) => Ok(HttpResponse::Ok().json({
    //         serde_json::json!({
    //             "message": "Your vote has been successfully cast.",
    //             "poll_id": poll_id_path,
    //             "option": option,
    //         })
    //     })),
    //     Err(_) => {
    //         // Restituisci l'errore in caso di fallimento
    //         Err(ApiErrorResponse::NotFound.into())
    //     }
    // }

    Ok(HttpResponse::Ok().json({
        serde_json::json!({
            "message": "Your vote has been successfully cast.",
            "poll_id": 10,
            "option": "ciao",
        })
    }))
}
