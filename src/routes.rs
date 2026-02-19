use axum::{
    Router,
    routing::{get, post, put, delete},
};
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
        .route("/users/email", get(user_handler::get_user_by_email))
        .route("/users/email", put(user_handler::update_user))
        .route("/users/email", delete(user_handler::delete_user))
        .layer(CompressionLayer::new())
        .with_state(pool)
}
