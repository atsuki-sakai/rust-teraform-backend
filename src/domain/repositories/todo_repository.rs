use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Todo;
use crate::shared::error::AppResult;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, todo: &Todo) -> AppResult<Todo>;
    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<Option<Todo>>;
    async fn find_all_by_user(&self, user_id: Uuid, limit: i64, offset: i64) -> AppResult<Vec<Todo>>;
    async fn count_by_user(&self, user_id: Uuid) -> AppResult<i64>;
    async fn update(&self, todo: &Todo) -> AppResult<Todo>;
    async fn delete(&self, id: Uuid, user_id: Uuid) -> AppResult<()>;
}
