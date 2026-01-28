use axum::{extract::State, http::StatusCode, Json};

use crate::application::dto::{AuthResponse, LoginRequest, RefreshRequest, RegisterRequest};
use crate::application::services::AuthService;
use crate::infrastructure::config::AppState;
use crate::shared::error::AppResult;

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already registered")
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<(StatusCode, Json<AuthResponse>)> {
    let service = AuthService::new(state.user_repository.clone(), state.jwt_config.clone());
    let response = service.register(request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let service = AuthService::new(state.user_repository.clone(), state.jwt_config.clone());
    let response = service.login(request).await?;
    Ok(Json(response))
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AuthResponse),
        (status = 401, description = "Invalid refresh token")
    ),
    tag = "auth"
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(request): Json<RefreshRequest>,
) -> AppResult<Json<AuthResponse>> {
    let service = AuthService::new(state.user_repository.clone(), state.jwt_config.clone());
    let response = service.refresh(request).await?;
    Ok(Json(response))
}
