use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreateTodoRequest, PaginationQuery, TodoListResponse, TodoResponse, UpdateTodoRequest,
};
use crate::domain::entities::{Todo, TodoTitle};
use crate::domain::repositories::TodoRepository;
use crate::shared::error::{AppError, AppResult};

pub struct TodoService {
    todo_repository: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(todo_repository: Arc<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        request: CreateTodoRequest,
    ) -> AppResult<TodoResponse> {
        let title = TodoTitle::new(request.title).map_err(AppError::Validation)?;
        let todo = Todo::new(user_id, title, request.description);
        let created = self.todo_repository.create(&todo).await?;
        Ok(TodoResponse::from(created))
    }

    pub async fn get(&self, user_id: Uuid, todo_id: Uuid) -> AppResult<TodoResponse> {
        let todo = self
            .todo_repository
            .find_by_id(todo_id.into(), user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;

        Ok(TodoResponse::from(todo))
    }

    pub async fn list(
        &self,
        user_id: Uuid,
        pagination: PaginationQuery,
    ) -> AppResult<TodoListResponse> {
        let todos = self
            .todo_repository
            .find_all_by_user(user_id, pagination.per_page(), pagination.offset())
            .await?;

        let total = self.todo_repository.count_by_user(user_id).await?;

        Ok(TodoListResponse {
            todos: todos.into_iter().map(TodoResponse::from).collect(),
            total,
            page: pagination.page(),
            per_page: pagination.per_page(),
        })
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        todo_id: Uuid,
        request: UpdateTodoRequest,
    ) -> AppResult<TodoResponse> {
        let mut todo = self
            .todo_repository
            .find_by_id(todo_id.into(), user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;

        let title = match request.title {
            Some(t) => Some(TodoTitle::new(t).map_err(AppError::Validation)?),
            None => None,
        };

        todo.update(title, request.description, request.completed);

        let updated = self.todo_repository.update(&todo).await?;
        Ok(TodoResponse::from(updated))
    }

    pub async fn delete(&self, user_id: Uuid, todo_id: Uuid) -> AppResult<()> {
        // Check if todo exists
        self.todo_repository
            .find_by_id(todo_id.into(), user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;

        self.todo_repository.delete(todo_id.into(), user_id).await
    }
}
