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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::UserId;
    use async_trait::async_trait;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        UserRepo {}

        #[async_trait]
        impl UserRepository for UserRepo {
            async fn create(&self, user: &User) -> AppResult<User>;
            async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;
            async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
        }
    }

    fn create_test_jwt_config() -> JwtConfig {
        JwtConfig {
            secret: "test-secret-key-for-unit-testing-at-least-32-chars".to_string(),
            access_token_expires_in: chrono::Duration::minutes(15),
            refresh_token_expires_in: chrono::Duration::days(7),
        }
    }

    #[tokio::test]
    async fn test_register_success() {
        // Arrange
        let mut mock_repo = MockUserRepo::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let user_id = UserId::new();
        let now = chrono::Utc::now();
        let expected_user = User {
            id: user_id,
            email: email.clone(),
            password_hash: password_hash.clone(),
            created_at: now,
            updated_at: now,
        };

        mock_repo
            .expect_find_by_email()
            .with(eq(email.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let expected_user_clone = expected_user.clone();
        mock_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(expected_user_clone.clone()));

        let jwt_config = create_test_jwt_config();
        let service = AuthService::new(Arc::new(mock_repo), jwt_config);

        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = service.register(request).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.token_type, "Bearer");
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_register_email_already_exists() {
        // Arrange
        let mut mock_repo = MockUserRepo::new();
        let email = Email::new("existing@example.com".to_string()).unwrap();
        let now = chrono::Utc::now();
        let existing_user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: PasswordHash::new("hashed".to_string()),
            created_at: now,
            updated_at: now,
        };

        mock_repo
            .expect_find_by_email()
            .with(eq(email.clone()))
            .times(1)
            .returning(move |_| Ok(Some(existing_user.clone())));

        let jwt_config = create_test_jwt_config();
        let service = AuthService::new(Arc::new(mock_repo), jwt_config);

        let request = RegisterRequest {
            email: "existing@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = service.register(request).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Conflict(_)));
    }

    #[tokio::test]
    async fn test_register_invalid_email() {
        // Arrange
        let mock_repo = MockUserRepo::new();
        let jwt_config = create_test_jwt_config();
        let service = AuthService::new(Arc::new(mock_repo), jwt_config);

        let request = RegisterRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = service.register(request).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[tokio::test]
    async fn test_login_success() {
        // Arrange
        let mut mock_repo = MockUserRepo::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password = "password123";
        let password_hash_str = hash_password(password).unwrap();
        let password_hash = PasswordHash::new(password_hash_str);
        let now = chrono::Utc::now();

        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: password_hash.clone(),
            created_at: now,
            updated_at: now,
        };

        mock_repo
            .expect_find_by_email()
            .with(eq(email.clone()))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));

        let jwt_config = create_test_jwt_config();
        let service = AuthService::new(Arc::new(mock_repo), jwt_config);

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: password.to_string(),
        };

        // Act
        let result = service.login(request).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.token_type, "Bearer");
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        // Arrange
        let mut mock_repo = MockUserRepo::new();
        let email = Email::new("notfound@example.com".to_string()).unwrap();

        mock_repo
            .expect_find_by_email()
            .with(eq(email.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let jwt_config = create_test_jwt_config();
        let service = AuthService::new(Arc::new(mock_repo), jwt_config);

        let request = LoginRequest {
            email: "notfound@example.com".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = service.login(request).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidCredentials));
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        // Arrange
        let mut mock_repo = MockUserRepo::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let correct_password = "correct_password";
        let password_hash_str = hash_password(correct_password).unwrap();
        let password_hash = PasswordHash::new(password_hash_str);
        let now = chrono::Utc::now();

        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: password_hash.clone(),
            created_at: now,
            updated_at: now,
        };

        mock_repo
            .expect_find_by_email()
            .with(eq(email.clone()))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));

        let jwt_config = create_test_jwt_config();
        let service = AuthService::new(Arc::new(mock_repo), jwt_config);

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrong_password".to_string(),
        };

        // Act
        let result = service.login(request).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::InvalidCredentials));
    }
}
