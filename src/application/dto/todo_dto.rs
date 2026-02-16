use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::Todo;

/// Page number newtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Page(i64);

impl Page {
    pub fn new(value: Option<i64>) -> Self {
        Self(value.unwrap_or(1).max(1))
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

/// Items per page newtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct PerPage(i64);

impl PerPage {
    pub fn new(value: Option<i64>) -> Self {
        Self(value.unwrap_or(20).clamp(1, 100))
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

/// Offset newtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Offset(i64);

impl Offset {
    pub fn from_page(page: Page, per_page: PerPage) -> Self {
        Self((page.value() - 1) * per_page.value())
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id.0,
            title: todo.title.value().to_string(),
            description: todo.description.map(|d| d.value().to_string()),
            completed: todo.completed.into(),
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoListResponse {
    pub todos: Vec<TodoResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl PaginationQuery {
    pub fn page(&self) -> Page {
        Page::new(self.page)
    }

    pub fn per_page(&self) -> PerPage {
        PerPage::new(self.per_page)
    }

    pub fn offset(&self) -> Offset {
        Offset::from_page(self.page(), self.per_page())
    }
}
