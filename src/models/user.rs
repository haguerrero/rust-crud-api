use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use async_graphql::SimpleObject;

/// A registered user returned by the API.
#[derive(Debug, Serialize, SimpleObject)]
pub struct UserResponse {
    /// Unique identifier (UUID) of the user.
    pub id: String,
    /// Email address of the user.
    pub email: String,
    /// Timestamp when the user was created.
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

/// Authentication response containing a JWT token.
#[derive(Debug, Serialize, SimpleObject)]
pub struct AuthResponse {
    /// JWT token to use in subsequent requests.
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct GetUserByEmailQuery {
    pub email: String,
}