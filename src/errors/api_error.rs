use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    EmailAlreadyExists,
    BadRequest(String),
    InternalServerError,
    InvalidCredentials,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::EmailAlreadyExists => {
                (StatusCode::CONFLICT, "Email already exists".to_string())
            }
            ApiError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}
