use sqlx::MySqlPool;

use crate::db::user_repository;
use crate::errors::api_error::ApiError;
use crate::models::user::UserResponse;

#[derive(Clone)]
pub struct UserService {
    pool: MySqlPool,
}

impl UserService {
    pub fn new(pool: MySqlPool) -> Self {
        UserService { pool }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, ApiError> {
        user_repository::get_users(&self.pool, None, None)
            .await
            .map_err(|_| ApiError::InternalServerError)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserResponse>, ApiError> {
        user_repository::find_user_by_email(&self.pool, email)
            .await
            .map_err(|_| ApiError::InternalServerError)
    }

    pub async fn get_users_paginated(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<UserResponse>, ApiError> {
        user_repository::get_users(&self.pool, limit, offset)
            .await
            .map_err(|_| ApiError::InternalServerError)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, ApiError> {
        let login_request = crate::models::user::LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };
        let auth_response = crate::services::auth_service::login(&self.pool, login_request).await?;
        Ok(auth_response.token)
    }
}
