use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::Arc;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::{
    config::Config, 
    db::mysql::create_pool,
};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::MySqlPool,
}

pub async fn run() {
    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await;
    let state = Arc::new(AppState { db: pool });

    // Route base
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(state);
    
    let addr = "0.0.0.0:3000";
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
    }

async fn health_check(
    State(state): State<Arc<AppState>>,
) -> &'static str {
    let _ = &state.db;
    "OK"
}