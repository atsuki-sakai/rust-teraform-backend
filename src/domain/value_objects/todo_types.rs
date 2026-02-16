use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

/// Todo ID newtype for type safety
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

/// Todo title newtype with validation
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

/// Todo description newtype with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(transparent)]
pub struct TodoDescription(String);

impl TodoDescription {
    pub fn new(description: String) -> Result<Self, String> {
        if description.len() > 1000 {
            return Err("Description cannot be longer than 1000 characters".to_string());
        }
        Ok(Self(description))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Completion status enum for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum CompletionStatus {
    #[default]
    Pending,
    Completed,
}

// Manual SQLx Type implementation for boolean mapping
impl sqlx::Type<sqlx::Postgres> for CompletionStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <bool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for CompletionStatus {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <bool as sqlx::Encode<sqlx::Postgres>>::encode(self.is_completed(), buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for CompletionStatus {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let completed = <bool as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(CompletionStatus::from(completed))
    }
}

impl CompletionStatus {
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }

    pub fn toggle(&self) -> Self {
        match self {
            Self::Pending => Self::Completed,
            Self::Completed => Self::Pending,
        }
    }
}

impl From<bool> for CompletionStatus {
    fn from(completed: bool) -> Self {
        if completed {
            Self::Completed
        } else {
            Self::Pending
        }
    }
}

impl From<CompletionStatus> for bool {
    fn from(status: CompletionStatus) -> bool {
        status.is_completed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_id_creation() {
        let id1 = TodoId::new();
        let id2 = TodoId::new();
        assert_ne!(id1, id2);
    }

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

    #[test]
    fn test_todo_description_validation() {
        // Valid description
        let valid = TodoDescription::new("Valid description".to_string());
        assert!(valid.is_ok());

        // Empty is valid
        let empty = TodoDescription::new("".to_string());
        assert!(empty.is_ok());

        // Too long
        let long = TodoDescription::new("a".repeat(1001));
        assert!(long.is_err());
        assert_eq!(
            long.unwrap_err(),
            "Description cannot be longer than 1000 characters"
        );
    }

    #[test]
    fn test_completion_status() {
        // Default is Pending
        assert_eq!(CompletionStatus::default(), CompletionStatus::Pending);

        // is_completed
        assert!(!CompletionStatus::Pending.is_completed());
        assert!(CompletionStatus::Completed.is_completed());

        // toggle
        assert_eq!(
            CompletionStatus::Pending.toggle(),
            CompletionStatus::Completed
        );
        assert_eq!(
            CompletionStatus::Completed.toggle(),
            CompletionStatus::Pending
        );

        // From bool
        assert_eq!(CompletionStatus::from(false), CompletionStatus::Pending);
        assert_eq!(CompletionStatus::from(true), CompletionStatus::Completed);

        // To bool
        assert!(!bool::from(CompletionStatus::Pending));
        assert!(bool::from(CompletionStatus::Completed));
    }
}
