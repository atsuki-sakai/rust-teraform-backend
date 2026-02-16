use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::value_objects::{CompletionStatus, TodoDescription, TodoId, TodoTitle, UserId};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Todo {
    pub id: TodoId,
    pub user_id: UserId,
    pub title: TodoTitle,
    pub description: Option<TodoDescription>,
    pub completed: CompletionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(
        user_id: UserId,
        title: TodoTitle,
        description: Option<TodoDescription>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: TodoId::new(),
            user_id,
            title,
            description,
            completed: CompletionStatus::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(
        &mut self,
        title: Option<TodoTitle>,
        description: Option<Option<TodoDescription>>,
        completed: Option<CompletionStatus>,
    ) {
        if let Some(t) = title {
            self.title = t;
        }
        if let Some(d) = description {
            self.description = d;
        }
        if let Some(c) = completed {
            self.completed = c;
        }
        self.updated_at = Utc::now();
    }

    pub fn toggle_completion(&mut self) {
        self.completed = self.completed.toggle();
        self.updated_at = Utc::now();
    }
}
