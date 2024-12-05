use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::routes::auth::signin::{Identity, TokenPair};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,     // Subject (e.g., user identifier)
    company: String, // Optional claim
    exp: usize,      // Expiration time (in seconds since epoch)
}

#[derive(Clone)]
pub struct JwtManager {
    access_secret: String,
    refresh_secret: String,
    access_token_exp: usize,  // Expiration time for access tokens in seconds
    refresh_token_exp: usize, // Expiration time for refresh tokens in seconds
}

impl JwtManager {
    /// Create a new JwtManager with secrets and expiration times
    pub fn new(access_secret: String, refresh_secret: String) -> Self {
        Self {
            access_secret,
            refresh_secret,
            access_token_exp: 15 * 60,            // Default 15 minutes
            refresh_token_exp: 30 * 24 * 60 * 60, // Default 30 days
        }
    }

    /// Generate a pair of JWT tokens (access + refresh)
    pub fn generate_token_pair(&self, identity: Identity) -> TokenPair {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        // Claims for access token
        let access_claims = Claims {
            sub: identity.address.clone(),
            company: "VoteChain".to_owned(),
            exp: now + self.access_token_exp,
        };

        // Claims for refresh token
        let refresh_claims = Claims {
            sub: identity.address.clone(),
            company: "VoteChain".to_owned(),
            exp: now + self.refresh_token_exp,
        };

        // Generate JWT access token
        let token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.access_secret.as_ref()),
        )
        .expect("Failed to generate access token");

        // Generate JWT refresh token
        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.refresh_secret.as_ref()),
        )
        .expect("Failed to generate refresh token");

        TokenPair {
            token,
            refresh_token,
        }
    }

    /// Verify a JWT token and return its claims
    pub fn verify_token(
        &self,
        token: &str,
        is_refresh: bool,
    ) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = if is_refresh {
            &self.refresh_secret
        } else {
            &self.access_secret
        };

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
