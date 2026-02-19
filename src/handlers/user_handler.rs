use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use crate::models::user::{
    CreateUser, 
    UserResponse,
    AuthResponse,
    LoginRequest,
    GetUserByEmailQuery
};

use serde::Deserialize;
use sqlx::MySqlPool;

use std::time::Instant;

use crate::db::user_repository;
use crate::errors::api_error::ApiError;
use axum::response::IntoResponse;
use crate::middleware::auth::AuthenticatedUser;

use crate::services::auth_service;
use crate::AppState;

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn get_users(
    State(state): State<AppState>,
    AuthenticatedUser(_claims): AuthenticatedUser,
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let limit = params.limit.unwrap_or(10000);
    let offset = params.offset.unwrap_or(0);

    let sql_start = Instant::now();
    let users = user_repository::get_users(&state.pool, Some(limit), Some(offset))
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let sql_duration = sql_start.elapsed();

    println!("SQL query time: {:?}", sql_duration);

    Ok(Json(users))
}

pub async fn get_user_by_email(
    State(state): State<AppState>,
    AuthenticatedUser(_claims): AuthenticatedUser,
    Query(params): Query<GetUserByEmailQuery>,
) -> Result<Json<UserResponse>, ApiError> {
    let email = params.email;
    let user = user_repository::find_user_by_email(&state.pool, &email)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(user))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, ApiError> {
    let user = user_repository::create_user(&state.pool, payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    AuthenticatedUser(_claims): AuthenticatedUser,
    Query(params): Query<GetUserByEmailQuery>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserResponse>, ApiError> {
    let email = params.email;
    let user = user_repository::update_user(&state.pool, &email, &payload.email).await?;

    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let response = auth_service::login(&state.pool, payload).await?;

    Ok(Json(response))
}

pub async fn delete_user(
    State(state): State<AppState>,
    AuthenticatedUser(_claims): AuthenticatedUser,
    Query(params): Query<GetUserByEmailQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let email = params.email;
    user_repository::delete_user(&state.pool, &email).await?;

    Ok(StatusCode::NO_CONTENT)
}

