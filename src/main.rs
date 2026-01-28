use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use rust_teraform_backend::infrastructure::config::AppState;
use rust_teraform_backend::presentation::openapi::ApiDoc;
use rust_teraform_backend::presentation::routes::{auth_routes, todo_routes};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize application state
    let state = AppState::new()
        .await
        .expect("Failed to initialize app state");

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", axum::routing::get(health_check))
        // API routes
        .nest("/api/v1/auth", auth_routes())
        .nest("/api/v1/todos", todo_routes(state.clone()))
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    // Get port from environment or default
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("\n{}", "=".repeat(50));
    println!("  Todo API Server Started");
    println!("{}", "=".repeat(50));
    println!("  Health Check: http://localhost:{}/health", port);
    println!("  Swagger UI:   http://localhost:{}/swagger-ui", port);
    println!("  API Base:     http://localhost:{}/api/v1", port);
    println!("{}\n", "=".repeat(50));

    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    tokio::net::TcpListener::bind(addr).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
