use axum::{routing::post, Router};

use crate::infrastructure::config::AppState;
use crate::presentation::handlers::auth_handlers;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handlers::register))
        .route("/login", post(auth_handlers::login))
        .route("/refresh", post(auth_handlers::refresh))
}
