use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: TokenType,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expires_in: Duration,
    pub refresh_token_expires_in: Duration,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self {
            secret,
            access_token_expires_in: Duration::minutes(15),
            refresh_token_expires_in: Duration::days(7),
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid, email: &str) -> AppResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            exp: (now + self.access_token_expires_in).timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Access,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(AppError::from)
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, email: &str) -> AppResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            exp: (now + self.refresh_token_expires_in).timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Refresh,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(AppError::from)
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
