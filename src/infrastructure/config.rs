use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::domain::repositories::{TodoRepository, UserRepository};
use crate::domain::services::auth_traits::{PasswordHasher, TokenGenerator};
use crate::infrastructure::auth::jwt::JwtConfig;
use crate::infrastructure::auth::password::Argon2PasswordHasher;
use crate::infrastructure::persistence::postgres::{
    PostgresTodoRepository, PostgresUserRepository,
};
use crate::shared::error::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub todo_repository: Arc<dyn TodoRepository>,
    pub user_repository: Arc<dyn UserRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub token_generator: Arc<dyn TokenGenerator>,
}

impl AppState {
    pub async fn new() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

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

        let password_hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::new());
        let token_generator: Arc<dyn TokenGenerator> = Arc::new(JwtConfig::from_env()?);

        Ok(Self {
            db_pool,
            todo_repository,
            user_repository,
            password_hasher,
            token_generator,
        })
    }
}
