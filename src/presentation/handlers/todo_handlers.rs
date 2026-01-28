use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::application::dto::{
    CreateTodoRequest, PaginationQuery, TodoListResponse, TodoResponse, UpdateTodoRequest,
};
use crate::application::services::TodoService;
use crate::infrastructure::auth::jwt::Claims;
use crate::infrastructure::config::AppState;
use crate::shared::error::AppResult;

/// List all todos for authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/todos",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 20, max: 100)")
    ),
    responses(
        (status = 200, description = "List of todos", body = TodoListResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "todos"
)]
pub async fn list_todos(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(pagination): Query<PaginationQuery>,
) -> AppResult<Json<TodoListResponse>> {
    let service = TodoService::new(state.todo_repository.clone());
    let response = service.list(claims.sub, pagination).await?;
    Ok(Json(response))
}

/// Get a specific todo
#[utoipa::path(
    get,
    path = "/api/v1/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Todo details", body = TodoResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Todo not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "todos"
)]
pub async fn get_todo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<TodoResponse>> {
    let service = TodoService::new(state.todo_repository.clone());
    let response = service.get(claims.sub, id).await?;
    Ok(Json(response))
}

/// Create a new todo
#[utoipa::path(
    post,
    path = "/api/v1/todos",
    request_body = CreateTodoRequest,
    responses(
        (status = 201, description = "Todo created", body = TodoResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "todos"
)]
pub async fn create_todo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateTodoRequest>,
) -> AppResult<(StatusCode, Json<TodoResponse>)> {
    let service = TodoService::new(state.todo_repository.clone());
    let response = service.create(claims.sub, request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// Update a todo
#[utoipa::path(
    put,
    path = "/api/v1/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    request_body = UpdateTodoRequest,
    responses(
        (status = 200, description = "Todo updated", body = TodoResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Todo not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "todos"
)]
pub async fn update_todo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTodoRequest>,
) -> AppResult<Json<TodoResponse>> {
    let service = TodoService::new(state.todo_repository.clone());
    let response = service.update(claims.sub, id, request).await?;
    Ok(Json(response))
}

/// Delete a todo
#[utoipa::path(
    delete,
    path = "/api/v1/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    responses(
        (status = 204, description = "Todo deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Todo not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "todos"
)]
pub async fn delete_todo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let service = TodoService::new(state.todo_repository.clone());
    service.delete(claims.sub, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
