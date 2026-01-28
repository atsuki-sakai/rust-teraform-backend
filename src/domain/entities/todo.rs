use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(user_id: Uuid, title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, title: Option<String>, description: Option<String>, completed: Option<bool>) {
        if let Some(t) = title {
            self.title = t;
        }
        if let Some(d) = description {
            self.description = Some(d);
        }
        if let Some(c) = completed {
            self.completed = c;
        }
        self.updated_at = Utc::now();
    }
}
