use std::sync::Arc;

use crate::application::dto::{AuthResponse, LoginRequest, RefreshRequest, RegisterRequest};
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::services::auth_traits::{PasswordHasher, TokenGenerator, TokenTypeEnum};
use crate::domain::value_objects::{Email, PasswordHash};
use crate::shared::error::{AppError, AppResult};

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_generator: Arc<dyn TokenGenerator>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_generator: Arc<dyn TokenGenerator>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
            token_generator,
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
        let password_hash_str = self.password_hasher.hash(&request.password)?;
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
        if !self
            .password_hasher
            .verify(&request.password, user.password_hash.value())?
        {
            return Err(AppError::InvalidCredentials);
        }

        // Generate tokens
        self.generate_tokens(&user)
    }

    pub async fn refresh(&self, request: RefreshRequest) -> AppResult<AuthResponse> {
        // Verify refresh token
        let claims = self.token_generator.verify_token(&request.refresh_token)?;

        // Check token type
        if claims.token_type != TokenTypeEnum::Refresh {
            return Err(AppError::Unauthorized);
        }

        // Find user
        let user = self
            .user_repository
            .find_by_id(claims.user_id)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Generate new tokens
        self.generate_tokens(&user)
    }

    fn generate_tokens(&self, user: &User) -> AppResult<AuthResponse> {
        let access_token = self
            .token_generator
            .generate_access_token(user.id, &user.email)?;
        let refresh_token = self
            .token_generator
            .generate_refresh_token(user.id, &user.email)?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.token_generator.access_token_expires_in(),
        })
    }
}
