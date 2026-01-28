use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::domain::repositories::{TodoRepository, UserRepository};
use crate::infrastructure::auth::jwt::JwtConfig;
use crate::infrastructure::persistence::postgres::{PostgresTodoRepository, PostgresUserRepository};
use crate::shared::error::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub todo_repository: Arc<dyn TodoRepository>,
    pub user_repository: Arc<dyn UserRepository>,
    pub jwt_config: JwtConfig,
}

impl AppState {
    pub async fn new() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        // Run migrations
        tracing::info!("Running database migrations...");
        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to run migrations");
        tracing::info!("Migrations completed successfully");

        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(PostgresTodoRepository::new(db_pool.clone()));
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(PostgresUserRepository::new(db_pool.clone()));

        let jwt_config = JwtConfig::from_env();

        Ok(Self {
            db_pool,
            todo_repository,
            user_repository,
            jwt_config,
        })
    }
}
