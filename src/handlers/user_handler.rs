use axum::{
    extract::{State, Query}, 
    Json,
    http::StatusCode
};

use crate::models::user::CreateUser;
use crate::models::user::UserResponse;
use serde::Deserialize;
use sqlx::MySqlPool;

use std::time::Instant;

use crate::db::user_repository;
use axum::response::IntoResponse;
use crate::errors::api_error::ApiError;

use crate::models::user::{LoginRequest, AuthResponse};
use crate::services::auth_service;

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn get_users(
    State(pool): State<MySqlPool>,
    Query(params): Query<Pagination>,
) -> Json<Vec<crate::models::user::UserResponse>> {
    
    let limit = params.limit.unwrap_or(10000);
    let offset = params.offset.unwrap_or(0);
    
    // Measure SQL query time
    let sql_start = Instant::now();
    let users = user_repository::get_users(
        &pool, 
        Some(limit), 
        Some(offset)
    )
    .await
    .expect("Failed to fetch users");
    let sql_duration = sql_start.elapsed();

    let user_start = Instant::now();
    let json = Json(users);
    let user_duration = user_start.elapsed();

    println!("SQL query time: {:?}", sql_duration);
    println!("User response time: {:?}", user_duration);

    let users = user_repository::get_users(
        &pool, 
        params.limit, 
        params.offset
    )
    .await
    .expect("Failed to fetch users");

    Json(users)
}

pub async fn create_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, ApiError> {

    let user = user_repository::create_user(&pool, payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[axum::debug_handler]
pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {

    let response = auth_service::login(&pool, payload).await?;

    Ok(Json(response))
}


// pub async fn get_users(
//     State(pool): State<MySqlPool>,
// ) -> Json<Vec<User>> {
//     let users = user_repository::get_users(&pool)
//         .await
//         .unwrap();

//     Json(users)
// }

