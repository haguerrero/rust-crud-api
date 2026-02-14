
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2
};
use sqlx::MySqlPool;
use uuid::Uuid;
use rand_core::OsRng;
use crate::errors::api_error::ApiError;
use sqlx::error::DatabaseError;

use crate::models::user::{UserResponse, CreateUser};

pub async fn create_user(
    pool: &MySqlPool,
    payload: CreateUser,
) -> Result<UserResponse, ApiError> {
    
    if payload.email.trim().is_empty() || payload.password.len() < 6 {
        return Err(ApiError::BadRequest("Invalid email or password".into()));
    }

    // UUID Binary format
    let user_id = Uuid::new_v4();
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| ApiError::InternalServerError)?
        .to_string();

    // Insert
    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES (?, ?, ?)
        "#,
        user_id.as_bytes().as_slice(),
        payload.email,
        password_hash
    )
    .execute(pool)
    .await;

    if let Err(err) = result {
    match err {
        sqlx::Error::Database(db_err) => {
            if db_err.code().as_deref() == Some("1062") {
                return Err(ApiError::EmailAlreadyExists);
            }
            return Err(ApiError::InternalServerError);
        }
        _ => return Err(ApiError::InternalServerError),
    }
}

 let row = sqlx::query!(
        r#"
        SELECT id, email, created_at
        FROM users
        WHERE id = ?
        "#,
        user_id.as_bytes().as_slice()
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    Ok(UserResponse {
        id: Uuid::from_slice(&row.id).unwrap().to_string(),
        email: row.email,
        created_at: row.created_at,
    })
}

pub async fn get_users(
    pool: &MySqlPool,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<UserResponse>, sqlx::Error> {

    let users = if let Some(limit) = limit {
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query!(
            r#"
            SELECT 
                id, 
                email, 
                created_at
            FROM users
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                let uuid = Uuid::from_slice(&row.id)
                    .expect("Invalid UUID id database");

                UserResponse {
                    id: uuid.to_string(),
                    email: row.email,
                    created_at: row.created_at,
                }
            })
            .collect()

    } else {

        let rows = sqlx::query!(
            r#"
            SELECT 
                id, 
                email, 
                created_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                let uuid = Uuid::from_slice(&row.id)
                    .expect("Invalid UUID id database");

                UserResponse {
                    id: uuid.to_string(),
                    email: row.email,
                    created_at: row.created_at,
                }
            })
            .collect()
    };

    Ok(users)
}
