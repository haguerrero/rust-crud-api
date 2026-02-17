use crate::errors::api_error::ApiError;
use crate::models::claims::Claims;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use std::future::Future;
use std::pin::Pin;

/// Esta struct representa al usuario autenticado
/// Envuelve Claims para que podamos acceder a los datos del usuario
pub struct AuthenticatedUser(pub Claims);

/// Implementamos el trait FromRequestParts
/// Esto permite que AuthenticatedUser sea usado como extractor en los handlers
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
            // Paso 1: Obtener el header Authorization
            let auth_header = parts
                .headers
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .ok_or(ApiError::Unauthorized)?;

            // Paso 2: Verificar que tenga el formato "Bearer <token>"
            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or(ApiError::Unauthorized)?;

            // Paso 3: Validar el token usando nuestra función
            let claims = crate::security::jwt::validate_token(token)
                .map_err(|_| ApiError::Unauthorized)?;

            // Paso 4: Retornar el usuario autenticado
            Ok(AuthenticatedUser(claims))
        }
    }
}