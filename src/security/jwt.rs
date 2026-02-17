use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use std::env;

use crate::models::claims::Claims;

pub fn get_secret() -> Vec<u8> {
    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes()
}

pub fn generate_token(user_id: &str, email: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
    };

    let secret = get_secret();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, Error> {
    let secret = get_secret();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
