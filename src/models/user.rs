use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use async_graphql::SimpleObject;

#[derive(Debug, Serialize, SimpleObject)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromRow)]
pub struct UserAuth {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct GetUserByEmailQuery {
    pub email: String,
}
