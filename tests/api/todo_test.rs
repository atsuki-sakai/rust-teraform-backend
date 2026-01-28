use axum::http::StatusCode;
use rust_teraform_backend::application::dto::{TodoListResponse, TodoResponse};
use uuid::Uuid;

use crate::common;

#[tokio::test]
async fn test_create_todo_success() {
    let (server, pool) = common::create_test_server().await;

    // Register and get token
    let auth = common::register_test_user(&server, "create_todo@example.com", "password123").await;

    // Create todo
    let response = server
        .post("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "title": "Test Todo",
            "description": "This is a test todo"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    let todo: TodoResponse = response.json();
    assert_eq!(todo.title, "Test Todo");
    assert_eq!(todo.description, Some("This is a test todo".to_string()));
    assert!(!todo.completed);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_todo_without_description() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "no_desc@example.com", "password123").await;

    let response = server
        .post("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "title": "Todo without description"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    let todo: TodoResponse = response.json();
    assert_eq!(todo.title, "Todo without description");
    assert_eq!(todo.description, None);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_todo_unauthorized() {
    let (server, pool) = common::create_test_server().await;

    let response = server
        .post("/api/v1/todos")
        .json(&serde_json::json!({
            "title": "Unauthorized Todo"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_list_todos_empty() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "list_empty@example.com", "password123").await;

    let response = server
        .get("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    response.assert_status_ok();

    let list: TodoListResponse = response.json();
    assert_eq!(list.todos.len(), 0);
    assert_eq!(list.total, 0);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_list_todos_with_pagination() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "pagination@example.com", "password123").await;

    // Create 5 todos
    for i in 1..=5 {
        server
            .post("/api/v1/todos")
            .add_header("Authorization", format!("Bearer {}", auth.access_token))
            .json(&serde_json::json!({
                "title": format!("Todo {}", i)
            }))
            .await
            .assert_status(StatusCode::CREATED);
    }

    // Get first page with 2 items
    let response = server
        .get("/api/v1/todos?page=1&per_page=2")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    response.assert_status_ok();

    let list: TodoListResponse = response.json();
    assert_eq!(list.todos.len(), 2);
    assert_eq!(list.total, 5);
    assert_eq!(list.page, 1);
    assert_eq!(list.per_page, 2);

    // Get second page
    let response = server
        .get("/api/v1/todos?page=2&per_page=2")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    let list: TodoListResponse = response.json();
    assert_eq!(list.todos.len(), 2);
    assert_eq!(list.page, 2);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_todo_success() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "get_todo@example.com", "password123").await;

    // Create todo
    let create_response = server
        .post("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "title": "Get Me",
            "description": "Find this todo"
        }))
        .await;

    let created_todo: TodoResponse = create_response.json();

    // Get todo
    let response = server
        .get(&format!("/api/v1/todos/{}", created_todo.id))
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    response.assert_status_ok();

    let todo: TodoResponse = response.json();
    assert_eq!(todo.id, created_todo.id);
    assert_eq!(todo.title, "Get Me");

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_todo_not_found() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "notfound@example.com", "password123").await;

    let random_id = Uuid::new_v4();
    let response = server
        .get(&format!("/api/v1/todos/{}", random_id))
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_update_todo_success() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "update@example.com", "password123").await;

    // Create todo
    let create_response = server
        .post("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "title": "Original Title",
            "description": "Original Description"
        }))
        .await;

    let created_todo: TodoResponse = create_response.json();

    // Update todo
    let response = server
        .put(&format!("/api/v1/todos/{}", created_todo.id))
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "title": "Updated Title",
            "description": "Updated Description",
            "completed": true
        }))
        .await;

    response.assert_status_ok();

    let todo: TodoResponse = response.json();
    assert_eq!(todo.title, "Updated Title");
    assert_eq!(todo.description, Some("Updated Description".to_string()));
    assert!(todo.completed);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_update_todo_partial() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "partial@example.com", "password123").await;

    // Create todo
    let create_response = server
        .post("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "title": "Keep This Title",
            "description": "Keep This Description"
        }))
        .await;

    let created_todo: TodoResponse = create_response.json();

    // Update only completed status
    let response = server
        .put(&format!("/api/v1/todos/{}", created_todo.id))
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "completed": true
        }))
        .await;

    response.assert_status_ok();

    let todo: TodoResponse = response.json();
    assert_eq!(todo.title, "Keep This Title"); // Unchanged
    assert!(todo.completed);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_delete_todo_success() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "delete@example.com", "password123").await;

    // Create todo
    let create_response = server
        .post("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .json(&serde_json::json!({
            "title": "Delete Me"
        }))
        .await;

    let created_todo: TodoResponse = create_response.json();

    // Delete todo
    let response = server
        .delete(&format!("/api/v1/todos/{}", created_todo.id))
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    response.assert_status(StatusCode::NO_CONTENT);

    // Verify deleted
    let get_response = server
        .get(&format!("/api/v1/todos/{}", created_todo.id))
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    get_response.assert_status(StatusCode::NOT_FOUND);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_delete_todo_not_found() {
    let (server, pool) = common::create_test_server().await;

    let auth = common::register_test_user(&server, "delete_notfound@example.com", "password123").await;

    let random_id = Uuid::new_v4();
    let response = server
        .delete(&format!("/api/v1/todos/{}", random_id))
        .add_header("Authorization", format!("Bearer {}", auth.access_token))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_cannot_access_other_users_todo() {
    let (server, pool) = common::create_test_server().await;

    // User 1 creates a todo
    let auth1 = common::register_test_user(&server, "user1@example.com", "password123").await;
    let create_response = server
        .post("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth1.access_token))
        .json(&serde_json::json!({
            "title": "User 1's Private Todo"
        }))
        .await;
    let todo: TodoResponse = create_response.json();

    // User 2 tries to access User 1's todo
    let auth2 = common::register_test_user(&server, "user2@example.com", "password123").await;

    // Get - should return 404 (not 403, to avoid leaking existence)
    let response = server
        .get(&format!("/api/v1/todos/{}", todo.id))
        .add_header("Authorization", format!("Bearer {}", auth2.access_token))
        .await;
    response.assert_status(StatusCode::NOT_FOUND);

    // Update - should return 404
    let response = server
        .put(&format!("/api/v1/todos/{}", todo.id))
        .add_header("Authorization", format!("Bearer {}", auth2.access_token))
        .json(&serde_json::json!({
            "title": "Hacked!"
        }))
        .await;
    response.assert_status(StatusCode::NOT_FOUND);

    // Delete - should return 404
    let response = server
        .delete(&format!("/api/v1/todos/{}", todo.id))
        .add_header("Authorization", format!("Bearer {}", auth2.access_token))
        .await;
    response.assert_status(StatusCode::NOT_FOUND);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_users_see_only_their_todos() {
    let (server, pool) = common::create_test_server().await;

    // User 1 creates todos
    let auth1 = common::register_test_user(&server, "isolation1@example.com", "password123").await;
    for i in 1..=3 {
        server
            .post("/api/v1/todos")
            .add_header("Authorization", format!("Bearer {}", auth1.access_token))
            .json(&serde_json::json!({
                "title": format!("User1 Todo {}", i)
            }))
            .await;
    }

    // User 2 creates todos
    let auth2 = common::register_test_user(&server, "isolation2@example.com", "password123").await;
    for i in 1..=2 {
        server
            .post("/api/v1/todos")
            .add_header("Authorization", format!("Bearer {}", auth2.access_token))
            .json(&serde_json::json!({
                "title": format!("User2 Todo {}", i)
            }))
            .await;
    }

    // User 1 should see only their 3 todos
    let response = server
        .get("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth1.access_token))
        .await;
    let list: TodoListResponse = response.json();
    assert_eq!(list.total, 3);
    for todo in &list.todos {
        assert!(todo.title.starts_with("User1"));
    }

    // User 2 should see only their 2 todos
    let response = server
        .get("/api/v1/todos")
        .add_header("Authorization", format!("Bearer {}", auth2.access_token))
        .await;
    let list: TodoListResponse = response.json();
    assert_eq!(list.total, 2);
    for todo in &list.todos {
        assert!(todo.title.starts_with("User2"));
    }

    common::cleanup_test_data(&pool).await;
}
