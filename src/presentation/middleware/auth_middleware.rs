use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::infrastructure::auth::jwt::{Claims, TokenType};
use crate::infrastructure::config::AppState;
use crate::shared::error::AppError;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = state.jwt_config.verify_token(token)?;

    // Verify token type is Access
    if claims.token_type != TokenType::Access {
        return Err(AppError::Unauthorized);
    }

    // Add claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// Extractor for getting claims from request
pub fn get_claims(request: &Request) -> Result<&Claims, AppError> {
    request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)
}
