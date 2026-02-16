use async_trait::async_trait;

use crate::domain::entities::User;
use crate::domain::value_objects::{Email, UserId};
use crate::shared::error::AppResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<User>;
    async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
}
