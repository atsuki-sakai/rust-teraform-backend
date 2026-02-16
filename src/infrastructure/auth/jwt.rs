use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::services::auth_traits::{self, TokenClaims, TokenTypeEnum};
use crate::domain::value_objects::{Email, UserId};
use crate::shared::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: UserId,
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
    pub fn from_env() -> AppResult<Self> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::Internal(anyhow::anyhow!("JWT_SECRET must be set")))?;

        if secret.len() < 32 {
            return Err(AppError::Internal(anyhow::anyhow!(
                "JWT_SECRET must be at least 32 characters long"
            )));
        }

        Ok(Self::new(secret))
    }

    pub fn new(secret: String) -> Self {
        Self {
            secret,
            access_token_expires_in: Duration::minutes(15),
            refresh_token_expires_in: Duration::days(7),
        }
    }

    fn create_token(
        &self,
        user_id: UserId,
        email: &str,
        token_type: TokenType,
        duration: Duration,
    ) -> AppResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            exp: (now + duration).timestamp(),
            iat: now.timestamp(),
            token_type,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(AppError::from)
    }

    pub fn generate_access_token(&self, user_id: UserId, email: &str) -> AppResult<String> {
        self.create_token(
            user_id,
            email,
            TokenType::Access,
            self.access_token_expires_in,
        )
    }

    pub fn generate_refresh_token(&self, user_id: UserId, email: &str) -> AppResult<String> {
        self.create_token(
            user_id,
            email,
            TokenType::Refresh,
            self.refresh_token_expires_in,
        )
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

impl auth_traits::TokenGenerator for JwtConfig {
    fn generate_access_token(&self, user_id: UserId, email: &Email) -> AppResult<String> {
        JwtConfig::generate_access_token(self, user_id, email.value())
    }

    fn generate_refresh_token(&self, user_id: UserId, email: &Email) -> AppResult<String> {
        JwtConfig::generate_refresh_token(self, user_id, email.value())
    }

    fn verify_token(&self, token: &str) -> AppResult<TokenClaims> {
        let claims = JwtConfig::verify_token(self, token)?;
        Ok(TokenClaims {
            user_id: claims.sub,
            email: claims.email,
            token_type: match claims.token_type {
                TokenType::Access => TokenTypeEnum::Access,
                TokenType::Refresh => TokenTypeEnum::Refresh,
            },
        })
    }

    fn access_token_expires_in(&self) -> i64 {
        self.access_token_expires_in.num_seconds()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
