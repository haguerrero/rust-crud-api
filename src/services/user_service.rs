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
}
