use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::routes::auth::signin::{Identity, TokenPair};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (e.g., user identifier)
    pub aud: String, // Audience (e.g., votechain client)
    pub exp: usize,  // Expiration time (in seconds since epoch)
}

#[derive(Clone)]
pub struct JwtManager {
    access_secret: String,
    refresh_secret: String,
    access_token_exp: usize,  // Expiration time for access tokens in seconds
    refresh_token_exp: usize, // Expiration time for refresh tokens in seconds
}

impl JwtManager {
    // Create a new JwtManager with secrets and expiration times
    pub fn new(access_secret: String, refresh_secret: String) -> Self {
        Self {
            access_secret,
            refresh_secret,
            access_token_exp: 15 * 60,            // Default 15 minutes
            refresh_token_exp: 30 * 24 * 60 * 60, // Default 30 days
        }
    }

    pub fn new_access_token_from_refresh(&self, refresh_token: &str) -> Option<String> {
        let decoded_token = self.decode_token(refresh_token, true).ok()?;

        // Create a new access token from the claims extracted from the refresh token
        let mut claims = decoded_token.claims.clone();

        // Update the expiration time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        // Compute the new expiration time
        let new_exp = now + self.access_token_exp;

        // Update the expiration time in the claims
        claims.exp = new_exp;

        // Generate the new access token
        Some(self.generate_token(claims, &self.access_secret))
    }

    // Generate a single JWT token
    pub fn generate_token(&self, claims: Claims, secret: &str) -> String {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .expect("Failed to generate token")
    }

    // Generate a pair of JWT tokens (access + refresh)
    pub fn generate_token_pair(&self, identity: Identity) -> TokenPair {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        // Claims for access token
        let access_claims = Claims {
            sub: identity.address.clone(),
            aud: "VoteChain".to_owned(),
            exp: now + self.access_token_exp,
        };

        // Claims for refresh token
        let refresh_claims = Claims {
            sub: identity.address.clone(),
            aud: "VoteChain".to_owned(),
            exp: now + self.refresh_token_exp,
        };

        // Generate access token
        let token = self.generate_token(access_claims, &self.access_secret);

        // Generate refresh token
        let refresh_token = self.generate_token(refresh_claims, &self.refresh_secret);

        TokenPair {
            token,
            refresh_token,
        }
    }

    /// Decode token with our custom validation routine
    pub fn decode_token(
        &self,
        token: &str,
        is_refresh: bool,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let secret = if is_refresh {
            &self.refresh_secret
        } else {
            &self.access_secret
        };

        // Define validation rules
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["VoteChain"]);
        validation.set_required_spec_claims(&["sub", "exp", "aud"]);

        // Decode the token, and pass validation rules
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
    }
}
