use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::application::dto::{
    AuthResponse, CreateTodoRequest, LoginRequest, RefreshRequest, RegisterRequest,
    TodoListResponse, TodoResponse, UpdateTodoRequest, UserResponse,
};
use crate::domain::entities::{Todo, User};
use crate::presentation::handlers::{auth_handlers, todo_handlers};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_handlers::register,
        auth_handlers::login,
        auth_handlers::refresh,
        todo_handlers::list_todos,
        todo_handlers::get_todo,
        todo_handlers::create_todo,
        todo_handlers::update_todo,
        todo_handlers::delete_todo,
    ),
    components(
        schemas(
            Todo,
            User,
            RegisterRequest,
            LoginRequest,
            RefreshRequest,
            AuthResponse,
            UserResponse,
            CreateTodoRequest,
            UpdateTodoRequest,
            TodoResponse,
            TodoListResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication API"),
        (name = "todos", description = "Todo management API")
    ),
    info(
        title = "Todo API",
        version = "1.0.0",
        description = "A simple Todo API built with Rust and Axum"
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
