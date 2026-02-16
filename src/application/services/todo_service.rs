use std::sync::Arc;

use crate::application::dto::{
    CreateTodoRequest, PaginationQuery, TodoListResponse, TodoResponse, UpdateTodoRequest,
};
use crate::domain::entities::Todo;
use crate::domain::repositories::TodoRepository;
use crate::domain::value_objects::{CompletionStatus, TodoDescription, TodoId, TodoTitle, UserId};
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
            .find_all_by_user(
                user_id,
                pagination.per_page().value(),
                pagination.offset().value(),
            )
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
            Some(desc) => Some(Some(
                TodoDescription::new(desc).map_err(AppError::Validation)?,
            )),
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        TodoRepo {}

        #[async_trait]
        impl TodoRepository for TodoRepo {
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
    }

    #[tokio::test]
    async fn test_create_todo_success() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let title = TodoTitle::new("Test Todo".to_string()).unwrap();
        let description = Some(TodoDescription::new("Test Description".to_string()).unwrap());
        let now = Utc::now();

        let expected_todo = Todo {
            id: TodoId::new(),
            user_id,
            title: title.clone(),
            description: description.clone(),
            completed: CompletionStatus::default(),
            created_at: now,
            updated_at: now,
        };

        let expected_todo_clone = expected_todo.clone();
        mock_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(expected_todo_clone.clone()));

        let service = TodoService::new(Arc::new(mock_repo));

        let request = CreateTodoRequest {
            title: "Test Todo".to_string(),
            description: Some("Test Description".to_string()),
        };

        // Act
        let result = service.create(user_id, request).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.title, "Test Todo");
        assert_eq!(response.description, Some("Test Description".to_string()));
    }

    #[tokio::test]
    async fn test_create_todo_title_too_long() {
        // Arrange
        let mock_repo = MockTodoRepo::new();
        let service = TodoService::new(Arc::new(mock_repo));
        let user_id = UserId::new();

        let request = CreateTodoRequest {
            title: "a".repeat(256), // Title exceeds maximum length
            description: None,
        };

        // Act
        let result = service.create(user_id, request).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[tokio::test]
    async fn test_get_todo_success() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let todo_id = TodoId::new();
        let title = TodoTitle::new("Test Todo".to_string()).unwrap();
        let now = Utc::now();

        let expected_todo = Todo {
            id: todo_id,
            user_id,
            title: title.clone(),
            description: None,
            completed: CompletionStatus::default(),
            created_at: now,
            updated_at: now,
        };

        mock_repo
            .expect_find_by_id()
            .with(eq(todo_id), eq(user_id))
            .times(1)
            .returning(move |_, _| Ok(Some(expected_todo.clone())));

        let service = TodoService::new(Arc::new(mock_repo));

        // Act
        let result = service.get(user_id, todo_id).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.title, "Test Todo");
    }

    #[tokio::test]
    async fn test_get_todo_not_found() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let todo_id = TodoId::new();

        mock_repo
            .expect_find_by_id()
            .with(eq(todo_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(None));

        let service = TodoService::new(Arc::new(mock_repo));

        // Act
        let result = service.get(user_id, todo_id).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_list_todos_success() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let now = Utc::now();

        let todos = vec![
            Todo {
                id: TodoId::new(),
                user_id,
                title: TodoTitle::new("Todo 1".to_string()).unwrap(),
                description: None,
                completed: CompletionStatus::default(),
                created_at: now,
                updated_at: now,
            },
            Todo {
                id: TodoId::new(),
                user_id,
                title: TodoTitle::new("Todo 2".to_string()).unwrap(),
                description: None,
                completed: CompletionStatus::default(),
                created_at: now,
                updated_at: now,
            },
        ];

        let todos_clone = todos.clone();
        mock_repo
            .expect_find_all_by_user()
            .with(eq(user_id), eq(10), eq(0))
            .times(1)
            .returning(move |_, _, _| Ok(todos_clone.clone()));

        mock_repo
            .expect_count_by_user()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Ok(2));

        let service = TodoService::new(Arc::new(mock_repo));

        let pagination = PaginationQuery {
            page: Some(1),
            per_page: Some(10),
        };

        // Act
        let result = service.list(user_id, pagination).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.todos.len(), 2);
        assert_eq!(response.total, 2);
        assert_eq!(response.page, 1);
        assert_eq!(response.per_page, 10);
    }

    #[tokio::test]
    async fn test_update_todo_success() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let todo_id = TodoId::new();
        let now = Utc::now();

        let existing_todo = Todo {
            id: todo_id,
            user_id,
            title: TodoTitle::new("Old Title".to_string()).unwrap(),
            description: None,
            completed: CompletionStatus::default(),
            created_at: now,
            updated_at: now,
        };

        let updated_todo = Todo {
            id: todo_id,
            user_id,
            title: TodoTitle::new("New Title".to_string()).unwrap(),
            description: Some(TodoDescription::new("New Description".to_string()).unwrap()),
            completed: CompletionStatus::Completed,
            created_at: now,
            updated_at: now,
        };

        mock_repo
            .expect_find_by_id()
            .with(eq(todo_id), eq(user_id))
            .times(1)
            .returning(move |_, _| Ok(Some(existing_todo.clone())));

        let updated_todo_clone = updated_todo.clone();
        mock_repo
            .expect_update()
            .times(1)
            .returning(move |_| Ok(updated_todo_clone.clone()));

        let service = TodoService::new(Arc::new(mock_repo));

        let request = UpdateTodoRequest {
            title: Some("New Title".to_string()),
            description: Some("New Description".to_string()),
            completed: Some(true),
        };

        // Act
        let result = service.update(user_id, todo_id, request).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.title, "New Title");
        assert_eq!(response.description, Some("New Description".to_string()));
        assert_eq!(response.completed, true);
    }

    #[tokio::test]
    async fn test_update_todo_not_found() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let todo_id = TodoId::new();

        mock_repo
            .expect_find_by_id()
            .with(eq(todo_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(None));

        let service = TodoService::new(Arc::new(mock_repo));

        let request = UpdateTodoRequest {
            title: Some("New Title".to_string()),
            description: None,
            completed: None,
        };

        // Act
        let result = service.update(user_id, todo_id, request).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_delete_todo_success() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let todo_id = TodoId::new();
        let now = Utc::now();

        let existing_todo = Todo {
            id: todo_id,
            user_id,
            title: TodoTitle::new("Test Todo".to_string()).unwrap(),
            description: None,
            completed: CompletionStatus::default(),
            created_at: now,
            updated_at: now,
        };

        mock_repo
            .expect_find_by_id()
            .with(eq(todo_id), eq(user_id))
            .times(1)
            .returning(move |_, _| Ok(Some(existing_todo.clone())));

        mock_repo
            .expect_delete()
            .with(eq(todo_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(()));

        let service = TodoService::new(Arc::new(mock_repo));

        // Act
        let result = service.delete(user_id, todo_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_todo_not_found() {
        // Arrange
        let mut mock_repo = MockTodoRepo::new();
        let user_id = UserId::new();
        let todo_id = TodoId::new();

        mock_repo
            .expect_find_by_id()
            .with(eq(todo_id), eq(user_id))
            .times(1)
            .returning(|_, _| Ok(None));

        let service = TodoService::new(Arc::new(mock_repo));

        // Act
        let result = service.delete(user_id, todo_id).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }
}
