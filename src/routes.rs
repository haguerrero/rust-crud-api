use axum::{Router, routing::{get, post}};
use sqlx::MySqlPool;
use tower_http::compression::CompressionLayer;

use crate::handlers::health;
use crate::handlers::user_handler;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/users", get(user_handler::get_users))
        .route("/users", post(user_handler::create_user))
        .route("/auth/login", post(user_handler::login))
        .layer(CompressionLayer::new())
        .with_state(pool)
}
