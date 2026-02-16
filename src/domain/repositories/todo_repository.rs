use async_trait::async_trait;

use crate::domain::entities::Todo;
use crate::domain::value_objects::{TodoId, UserId};
use crate::shared::error::AppResult;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, todo: &Todo) -> AppResult<Todo>;
    async fn find_by_id(&self, id: TodoId, user_id: UserId) -> AppResult<Option<Todo>>;
    async fn find_all_by_user(
        &self,
        user_id: UserId,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<Todo>>;
    async fn count_by_user(&self, user_id: UserId) -> AppResult<i64>;
    async fn update(&self, todo: &Todo) -> AppResult<Todo>;
    async fn delete(&self, id: TodoId, user_id: UserId) -> AppResult<()>;
}
