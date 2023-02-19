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
    // TokenCreation,
    InvalidToken,
    NoToken,
    CannotEncodeToken(jsonwebtoken::errors::Error),
    UserNotFound,
    AccessingUnauthorisedResources,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        // IntoResponseを実装している(StatusCode, &str)に詰め替える
        let (status, error_message) = match self {
            CustomError::WrongCredentials => (StatusCode::UNAUTHORIZED, "認証情報が誤っています。"),
            CustomError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "認証情報が不足しています。")
            }
            // CustomError::TokenCreation => {
            //     (StatusCode::INTERNAL_SERVER_ERROR, "トークンの生成に失敗しました。")
            // }
            CustomError::InvalidToken => (StatusCode::BAD_REQUEST, "不正なトークンです。"),
            CustomError::NoToken => (StatusCode::UNAUTHORIZED, "トークンがありません。"),
            CustomError::CannotEncodeToken(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "トークンのデコードに失敗しました。",
            ),
            CustomError::UserNotFound => {
                (StatusCode::UNPROCESSABLE_ENTITY, "ユーザは存在しません。")
            }
            CustomError::AccessingUnauthorisedResources => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "権限の無いリソースへのアクセスです。",
            ),
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
