use std::sync::Arc;

use crate::application::dto::{AuthResponse, LoginRequest, RefreshRequest, RegisterRequest};
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::{Email, PasswordHash};
use crate::infrastructure::auth::jwt::{JwtConfig, TokenType};
use crate::infrastructure::auth::password::{hash_password, verify_password};
use crate::shared::error::{AppError, AppResult};

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    jwt_config: JwtConfig,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_config: JwtConfig) -> Self {
        Self {
            user_repository,
            jwt_config,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> AppResult<AuthResponse> {
        // Validate and create Email
        let email = Email::new(request.email).map_err(AppError::Validation)?;

        // Check if user already exists
        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Hash password
        let password_hash_str = hash_password(&request.password)?;
        let password_hash = PasswordHash::new(password_hash_str);

        // Create user
        let user = User::new(email, password_hash);
        let created_user = self.user_repository.create(&user).await?;

        // Generate tokens
        self.generate_tokens(&created_user)
    }

    pub async fn login(&self, request: LoginRequest) -> AppResult<AuthResponse> {
        // Validate Email
        let email = Email::new(request.email).map_err(AppError::Validation)?;

        // Find user
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        if !verify_password(&request.password, user.password_hash.value())? {
            return Err(AppError::InvalidCredentials);
        }

        // Generate tokens
        self.generate_tokens(&user)
    }

    pub async fn refresh(&self, request: RefreshRequest) -> AppResult<AuthResponse> {
        // Verify refresh token
        let claims = self.jwt_config.verify_token(&request.refresh_token)?;

        // Check token type
        if claims.token_type != TokenType::Refresh {
            return Err(AppError::Unauthorized);
        }

        // Find user
        let user = self
            .user_repository
            .find_by_id(claims.sub)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Generate new tokens
        self.generate_tokens(&user)
    }

    fn generate_tokens(&self, user: &User) -> AppResult<AuthResponse> {
        let access_token = self
            .jwt_config
            .generate_access_token(user.id, user.email.value())?;
        let refresh_token = self
            .jwt_config
            .generate_refresh_token(user.id, user.email.value())?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_config.access_token_expires_in.num_seconds(),
        })
    }
}
