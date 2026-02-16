use std::sync::Arc;

use crate::application::dto::{
    CreateTodoRequest, PaginationQuery, TodoListResponse, TodoResponse, UpdateTodoRequest,
};
use crate::domain::entities::Todo;
use crate::domain::repositories::TodoRepository;
use crate::domain::value_objects::{
    CompletionStatus, TodoDescription, TodoId, TodoTitle, UserId,
};
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
        user_id: UserId,
        request: CreateTodoRequest,
    ) -> AppResult<TodoResponse> {
        let title = TodoTitle::new(request.title).map_err(AppError::Validation)?;
        let description = request
            .description
            .map(|d| TodoDescription::new(d).map_err(AppError::Validation))
            .transpose()?;
        let todo = Todo::new(user_id, title, description);
        let created = self.todo_repository.create(&todo).await?;
        Ok(TodoResponse::from(created))
    }

    pub async fn get(&self, user_id: UserId, todo_id: TodoId) -> AppResult<TodoResponse> {
        let todo = self
            .todo_repository
            .find_by_id(todo_id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;

        Ok(TodoResponse::from(todo))
    }

    pub async fn list(
        &self,
        user_id: UserId,
        pagination: PaginationQuery,
    ) -> AppResult<TodoListResponse> {
        let todos = self
            .todo_repository
            .find_all_by_user(user_id, pagination.per_page().value(), pagination.offset().value())
            .await?;

        let total = self.todo_repository.count_by_user(user_id).await?;

        Ok(TodoListResponse {
            todos: todos.into_iter().map(TodoResponse::from).collect(),
            total,
            page: pagination.page().value(),
            per_page: pagination.per_page().value(),
        })
    }

    pub async fn update(
        &self,
        user_id: UserId,
        todo_id: TodoId,
        request: UpdateTodoRequest,
    ) -> AppResult<TodoResponse> {
        let mut todo = self
            .todo_repository
            .find_by_id(todo_id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;

        let title = request
            .title
            .map(|t| TodoTitle::new(t).map_err(AppError::Validation))
            .transpose()?;

        let description = match request.description {
            Some(desc) => Some(Some(TodoDescription::new(desc).map_err(AppError::Validation)?)),
            None => None,
        };

        let completed = request.completed.map(CompletionStatus::from);

        todo.update(title, description, completed);

        let updated = self.todo_repository.update(&todo).await?;
        Ok(TodoResponse::from(updated))
    }

    pub async fn delete(&self, user_id: UserId, todo_id: TodoId) -> AppResult<()> {
        // Check if todo exists
        self.todo_repository
            .find_by_id(todo_id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;

        self.todo_repository.delete(todo_id, user_id).await
    }
}
