use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::infrastructure::config::AppState;
use crate::presentation::handlers::todo_handlers;
use crate::presentation::middleware::auth_middleware;

pub fn todo_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(todo_handlers::list_todos))
        .route("/", post(todo_handlers::create_todo))
        .route("/{id}", get(todo_handlers::get_todo))
        .route("/{id}", put(todo_handlers::update_todo))
        .route("/{id}", delete(todo_handlers::delete_todo))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}
