use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::User;
use crate::shared::error::AppResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<User>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
}
