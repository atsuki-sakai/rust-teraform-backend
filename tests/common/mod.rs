use axum::Router;
use axum_test::TestServer;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use rust_teraform_backend::application::dto::AuthResponse;
use rust_teraform_backend::domain::repositories::{TodoRepository, UserRepository};
use rust_teraform_backend::infrastructure::auth::jwt::JwtConfig;
use rust_teraform_backend::infrastructure::config::AppState;
use rust_teraform_backend::infrastructure::persistence::postgres::{
    PostgresTodoRepository, PostgresUserRepository,
};
use rust_teraform_backend::presentation::routes::{auth_routes, todo_routes};

/// Create a test database pool
pub async fn create_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://todo_user:todo_password@localhost:5433/todo_db".to_string()
    });

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

/// Clean up test data
pub async fn cleanup_test_data(pool: &PgPool) {
    sqlx::query("DELETE FROM todos")
        .execute(pool)
        .await
        .expect("Failed to clean todos");

    sqlx::query("DELETE FROM refresh_tokens")
        .execute(pool)
        .await
        .expect("Failed to clean refresh_tokens");

    sqlx::query("DELETE FROM users")
        .execute(pool)
        .await
        .expect("Failed to clean users");
}

/// Create test application state
pub async fn create_test_state(pool: PgPool) -> AppState {
    let todo_repository: Arc<dyn TodoRepository> =
        Arc::new(PostgresTodoRepository::new(pool.clone()));
    let user_repository: Arc<dyn UserRepository> =
        Arc::new(PostgresUserRepository::new(pool.clone()));

    let jwt_config = JwtConfig {
        secret: "test-secret-key-for-testing-only".to_string(),
        access_token_expires_in: chrono::Duration::minutes(15),
        refresh_token_expires_in: chrono::Duration::days(7),
    };

    AppState {
        db_pool: pool,
        todo_repository,
        user_repository,
        jwt_config,
    }
}

/// Create test server with full application
pub async fn create_test_server() -> (TestServer, PgPool) {
    dotenvy::dotenv().ok();

    let pool = create_test_pool().await;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Clean up before test
    cleanup_test_data(&pool).await;

    let state = create_test_state(pool.clone()).await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .nest("/api/v1/auth", auth_routes())
        .nest("/api/v1/todos", todo_routes(state.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    let server = TestServer::new(app).expect("Failed to create test server");

    (server, pool)
}

async fn health_check() -> &'static str {
    "OK"
}

/// Helper to register a test user and get auth tokens
#[allow(dead_code)]
pub async fn register_test_user(server: &TestServer, email: &str, password: &str) -> AuthResponse {
    let response = server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status_success();
    response.json::<AuthResponse>()
}

/// Helper to login and get auth tokens
#[allow(dead_code)]
pub async fn login_test_user(server: &TestServer, email: &str, password: &str) -> AuthResponse {
    let response = server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status_success();
    response.json::<AuthResponse>()
}
