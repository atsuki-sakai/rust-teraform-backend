use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(transparent)]
pub struct TodoId(pub Uuid);

impl TodoId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TodoId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for TodoId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(transparent)]
pub struct TodoTitle(String);

impl TodoTitle {
    pub fn new(title: String) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 255 {
            return Err("Title cannot be longer than 255 characters".to_string());
        }
        Ok(Self(title))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Todo {
    pub id: TodoId,
    pub user_id: Uuid,
    pub title: TodoTitle,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(user_id: Uuid, title: TodoTitle, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: TodoId::new(),
            user_id,
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(
        &mut self,
        title: Option<TodoTitle>,
        description: Option<String>,
        completed: Option<bool>,
    ) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_title_validation() {
        // Valid title
        let valid = TodoTitle::new("Valid Title".to_string());
        assert!(valid.is_ok());
        assert_eq!(valid.unwrap().value(), "Valid Title");

        // Empty title
        let empty = TodoTitle::new("".to_string());
        assert!(empty.is_err());
        assert_eq!(empty.unwrap_err(), "Title cannot be empty");

        // Whitespace only
        let whitespace = TodoTitle::new("   ".to_string());
        assert!(whitespace.is_err());
        assert_eq!(whitespace.unwrap_err(), "Title cannot be empty");

        // Too long
        let long = TodoTitle::new("a".repeat(256));
        assert!(long.is_err());
        assert_eq!(
            long.unwrap_err(),
            "Title cannot be longer than 255 characters"
        );
    }
}
