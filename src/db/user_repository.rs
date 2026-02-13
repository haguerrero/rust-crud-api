
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2
};
use sqlx::MySqlPool;
use uuid::Uuid;
use rand_core::OsRng;

use crate::models::user::{UserResponse, CreateUser};

pub async fn create_user(
    pool: &MySqlPool,
    payload: CreateUser,
) -> Result<UserResponse, sqlx::Error> {
    if payload.email.trim().is_empty() || payload.password.len() < 6 {
        return Err(sqlx::Error::Protocol("Invalid email or password".into()));
    }

    // UUID Binary format
    let user_id = Uuid::new_v4();
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);
    
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
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

    match result {
        Ok(_) => {
    let row = sqlx::query!(
        r#"
        SELECT id, email, created_at
        FROM users
        WHERE id = ?
        "#,
        user_id.as_bytes().as_slice()
    )
    .fetch_one(pool)
    .await?;

    Ok(UserResponse {
        id: Uuid::from_slice(&row.id).unwrap().to_string(),
        email: row.email,
        created_at: row.created_at,
    })
}
        Err(sqlx::Error::Database(db_err)) => {
        if db_err.code().unwrap_or_default() == "1062" {
            return Err(sqlx::Error::RowNotFound);
        }
        Err(sqlx::Error::Database(db_err))
    }    
    Err(e) => Err(e),
}
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
