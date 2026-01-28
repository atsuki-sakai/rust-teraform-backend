use axum::http::StatusCode;
use rust_teraform_backend::application::dto::AuthResponse;

use crate::common;

#[tokio::test]
async fn test_register_success() {
    let (server, pool) = common::create_test_server().await;

    let response = server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    let auth: AuthResponse = response.json();
    assert!(!auth.access_token.is_empty());
    assert!(!auth.refresh_token.is_empty());
    assert_eq!(auth.token_type, "Bearer");
    assert!(auth.expires_in > 0);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_register_duplicate_email_fails() {
    let (server, pool) = common::create_test_server().await;

    // First registration
    server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "duplicate@example.com",
            "password": "password123"
        }))
        .await
        .assert_status(StatusCode::CREATED);

    // Second registration with same email
    let response = server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "duplicate@example.com",
            "password": "different_password"
        }))
        .await;

    response.assert_status(StatusCode::CONFLICT);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_login_success() {
    let (server, pool) = common::create_test_server().await;

    // Register first
    common::register_test_user(&server, "login@example.com", "password123").await;

    // Login
    let response = server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": "login@example.com",
            "password": "password123"
        }))
        .await;

    response.assert_status_ok();

    let auth: AuthResponse = response.json();
    assert!(!auth.access_token.is_empty());
    assert!(!auth.refresh_token.is_empty());

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_login_invalid_password_fails() {
    let (server, pool) = common::create_test_server().await;

    // Register first
    common::register_test_user(&server, "invalid@example.com", "correct_password").await;

    // Login with wrong password
    let response = server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": "invalid@example.com",
            "password": "wrong_password"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_login_nonexistent_user_fails() {
    let (server, pool) = common::create_test_server().await;

    let response = server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": "nonexistent@example.com",
            "password": "password123"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_refresh_token_success() {
    let (server, pool) = common::create_test_server().await;

    // Register and get tokens
    let auth = common::register_test_user(&server, "refresh@example.com", "password123").await;

    // Wait a second to ensure different token timestamps
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Refresh token
    let response = server
        .post("/api/v1/auth/refresh")
        .json(&serde_json::json!({
            "refresh_token": auth.refresh_token
        }))
        .await;

    response.assert_status_ok();

    let new_auth: AuthResponse = response.json();
    assert!(!new_auth.access_token.is_empty());
    assert!(!new_auth.refresh_token.is_empty());
    // New tokens should be different (different timestamp)
    assert_ne!(new_auth.access_token, auth.access_token);
    // Refresh token should also be rotated
    assert_ne!(new_auth.refresh_token, auth.refresh_token);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_refresh_with_invalid_token_fails() {
    let (server, pool) = common::create_test_server().await;

    let response = server
        .post("/api/v1/auth/refresh")
        .json(&serde_json::json!({
            "refresh_token": "invalid.token.here"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    common::cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_refresh_with_access_token_fails() {
    let (server, pool) = common::create_test_server().await;

    // Register and get tokens
    let auth = common::register_test_user(&server, "access@example.com", "password123").await;

    // Try to use access token as refresh token
    let response = server
        .post("/api/v1/auth/refresh")
        .json(&serde_json::json!({
            "refresh_token": auth.access_token
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);

    common::cleanup_test_data(&pool).await;
}
