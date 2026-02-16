use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

/// User ID newtype for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Email newtype with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(transparent)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        let email = email.trim().to_lowercase();

        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err("Invalid email format".to_string());
        }

        if !parts[1].contains('.') {
            return Err("Invalid email domain".to_string());
        }

        Ok(Self(email))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Password hash newtype (not exposed in serialization)
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_creation() {
        let id1 = UserId::new();
        let id2 = UserId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_email_validation() {
        // Valid email
        let valid = Email::new("user@example.com".to_string());
        assert!(valid.is_ok());
        assert_eq!(valid.unwrap().value(), "user@example.com");

        // Uppercase should be normalized
        let uppercase = Email::new("User@Example.COM".to_string());
        assert!(uppercase.is_ok());
        assert_eq!(uppercase.unwrap().value(), "user@example.com");

        // Empty email
        let empty = Email::new("".to_string());
        assert!(empty.is_err());

        // Missing @
        let no_at = Email::new("userexample.com".to_string());
        assert!(no_at.is_err());

        // Missing domain
        let no_domain = Email::new("user@".to_string());
        assert!(no_domain.is_err());

        // Missing local part
        let no_local = Email::new("@example.com".to_string());
        assert!(no_local.is_err());

        // Missing TLD
        let no_tld = Email::new("user@example".to_string());
        assert!(no_tld.is_err());
    }

    #[test]
    fn test_password_hash() {
        let hash = PasswordHash::new("hashed_password".to_string());
        assert_eq!(hash.value(), "hashed_password");
    }
}
