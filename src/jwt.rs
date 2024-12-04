use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtClaims {
    pub poll_id: u32,
    pub option: String,
    pub wallet: String,
    pub signature: String,
    pub exp: u64,
}

pub fn decode_token(token: &str) -> Result<JwtClaims, String> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| "Missing JWT_SECRET".to_string())?;
    let key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    decode::<JwtClaims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|err| format!("Invalid token: {}", err))
}
