// use axum::{
//     extract::State,
//     http::StatusCode,
//     Json,
// };

// use sqlx::MySqlPool;

// pub mod user_handler;

// use crate::{
//     db::repository,
//     models::user::{CreateUser, User},
// };

// pub async fn create_user_handler(
//     State(pool): State<MySqlPool>,
//     Json(payload): Json<CreateUser>,
// ) -> Result<Json<User>, StatusCode> {
//     let user = user_repository::create_user(&pool, payload)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(Json(user))
// }

// pub async fn get_users_handler(
//     State(pool): State<MySqlPool>,
// ) -> Result<Json<Vec<User>>, StatusCode> {
//     let users = user_repository::get_users(&pool)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(Json(users))
// }

pub mod health;
pub mod user_handler;
