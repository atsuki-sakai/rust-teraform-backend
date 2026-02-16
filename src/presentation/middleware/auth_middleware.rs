use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::domain::services::auth_traits::TokenTypeEnum;
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

    let token_claims = state.token_generator.verify_token(token)?;

    // Verify token type is Access
    if token_claims.token_type != TokenTypeEnum::Access {
        return Err(AppError::Unauthorized);
    }

    // Convert TokenClaims to Infrastructure Claims for backward compatibility
    let claims = Claims {
        sub: token_claims.user_id,
        email: token_claims.email,
        exp: 0, // Not needed for handlers
        iat: 0, // Not needed for handlers
        token_type: TokenType::Access,
    };

    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
