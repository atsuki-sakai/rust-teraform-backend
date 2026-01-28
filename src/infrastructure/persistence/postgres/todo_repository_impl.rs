use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::{Todo, TodoId};
use crate::domain::repositories::TodoRepository;
use crate::shared::error::AppResult;

pub struct PostgresTodoRepository {
    pool: PgPool,
}

impl PostgresTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn create(&self, todo: &Todo) -> AppResult<Todo> {
        let created = sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (id, user_id, title, description, completed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, title, description, completed, created_at, updated_at
            "#,
        )
        .bind(todo.id)
        .bind(todo.user_id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed)
        .bind(todo.created_at)
        .bind(todo.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    async fn find_by_id(&self, id: TodoId, user_id: Uuid) -> AppResult<Option<Todo>> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, user_id, title, description, completed, created_at, updated_at
            FROM todos
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(todo)
    }

    async fn find_all_by_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<Todo>> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, user_id, title, description, completed, created_at, updated_at
            FROM todos
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    async fn count_by_user(&self, user_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM todos
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    async fn update(&self, todo: &Todo) -> AppResult<Todo> {
        let updated = sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos
            SET title = $1, description = $2, completed = $3, updated_at = $4
            WHERE id = $5 AND user_id = $6
            RETURNING id, user_id, title, description, completed, created_at, updated_at
            "#,
        )
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed)
        .bind(todo.updated_at)
        .bind(todo.id)
        .bind(todo.user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    async fn delete(&self, id: TodoId, user_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM todos
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
