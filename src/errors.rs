use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum CustomError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    CannotEncodeToken(jsonwebtoken::errors::Error),
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        // IntoResponseを実装している(StatusCode, &str)に詰め替える
        let (status, error_message) = match self {
            CustomError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            CustomError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            CustomError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            CustomError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            CustomError::CannotEncodeToken(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Token encode error")
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
