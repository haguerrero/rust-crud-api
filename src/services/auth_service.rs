use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::{
    db::user_repository,
    errors::api_error::ApiError,
    models::user::{AuthResponse, LoginRequest},
};

use sqlx::MySqlPool;

pub async fn login(
    pool: &MySqlPool,
    payload: LoginRequest,
) -> Result<AuthResponse, ApiError> {

    let user = user_repository::get_user_by_email(pool, &payload.email)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    let user = match user {
        Some(user) => user,
        None => return Err(ApiError::InvalidCredentials),
    };

    let password = payload.password.clone();
let hash = user.password_hash.clone();

let valid = tokio::task::spawn_blocking(move || {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|_| ())?;

    Ok::<bool, ()>(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    )
})
.await
.map_err(|_| ApiError::InternalServerError)?
.map_err(|_| ApiError::InternalServerError)?;

    
    if !valid {
        return Err(ApiError::InvalidCredentials);
    }
    
    Ok(AuthResponse {
        token: "temporary_token".to_string(), //falta token JWT
    })
}
