use crate::errors::api_error::ApiError;
use crate::models::claims::Claims;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::future::Future;
use std::pin::Pin;

pub struct AuthenticatedUser(pub Claims);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let auth_header = parts
                .headers
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .ok_or(ApiError::Unauthorized)?;

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or(ApiError::Unauthorized)?;

            let claims =
                crate::security::jwt::validate_token(token).map_err(|_| ApiError::Unauthorized)?;

            Ok(AuthenticatedUser(claims))
        }
    }
}
