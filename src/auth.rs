use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,     // Subject (e.g., user identifier)
    company: String, // Optional claim
    exp: usize,      // Expiration time (in seconds since epoch)
}

pub struct Identity {
    pub address: String, // User's blockchain address
                         // pub _chain_id: String, // Blockchain chain ID
}

pub struct SecretPair {
    pub secret: String,         // Secret for access token
    pub refresh_secret: String, // Secret for refresh token
}

pub struct TokenPair {
    pub token: String,         // Access token
    pub refresh_token: String, // Refresh token
}

pub fn generate_jwt_token_pair(data: Identity, secrets: SecretPair) -> TokenPair {
    // Set expiration times (e.g., 15 minutes for access, 30 days for refresh)
    let access_token_exp = 15 * 60; // 15 minutes in seconds
    let refresh_token_exp = 30 * 24 * 60 * 60; // 30 days in seconds

    // Get the current time as seconds since epoch
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    // Claims for access token
    let access_claims = Claims {
        sub: data.address.clone(),
        company: "YourCompany".to_owned(),
        exp: now + access_token_exp, // Expiration in 15 minutes
    };

    // Claims for refresh token
    let refresh_claims = Claims {
        sub: data.address.clone(),
        company: "YourCompany".to_owned(),
        exp: now + refresh_token_exp, // Expiration in 30 days
    };

    // Generate JWT access token
    let token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(secrets.secret.as_ref()),
    )
    .expect("Failed to generate access token");

    // Generate JWT refresh token
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(secrets.refresh_secret.as_ref()),
    )
    .expect("Failed to generate refresh token");

    // Return token pair
    TokenPair {
        token,
        refresh_token,
    }
}
