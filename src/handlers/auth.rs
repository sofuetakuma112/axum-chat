use std::sync::Arc;

use axum::{
    extract::{self, State},
    http::{header, StatusCode},
    response::{self, IntoResponse},
};
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    entities::users::User as UserEntity, errors::CustomError, jwt::JWT,
    repositories::user::UserRepository, request::Claims, views::user::User, AppState, KEYS,
};

#[axum_macros::debug_handler]
pub async fn get_user(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if let Some(user) = state.user_repository.find_by_user_id(claims.user_id).await {
        let user_view: User = user.into();
        Ok((StatusCode::OK, response::Json(json!({ "user": user_view }))))
    } else {
        Err(CustomError::UserNotFound)
    }
}

// アカウント新規作成
#[axum_macros::debug_handler]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<SignUpPayload>,
) -> Result<impl IntoResponse, CustomError> {
    // ユーザーがクレデンシャルを送信したかどうかを確認する
    if payload.email.is_empty() || payload.password.is_empty() || payload.name.is_empty() {
        return Err(CustomError::MissingCredentials);
    }

    let new_user = UserEntity::create(&payload.email, &payload.password, &payload.name);
    // TODO: 既に登録済みのEmailかチェックする
    let saved_user = state.user_repository.store(&new_user).await; // 新規ユーザーをDBに保存

    let jwt = create_token(&saved_user)?;

    Ok((
        StatusCode::CREATED,
        [(header::SET_COOKIE, jwt.cookie())],
        response::Json(json!({ "message": "ユーザが作成されました。" })),
    ))
}

// ログイン
#[axum_macros::debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<SignInPayload>,
) -> Result<impl IntoResponse, CustomError> {
    if let Some(user) = state.user_repository.find_by_email(&payload.email).await {
        if !user.matches_password(&payload.password) {
            return Err(CustomError::WrongCredentials);
        }

        let jwt = create_token(&user)?;

        Ok((
            StatusCode::OK,
            [(header::SET_COOKIE, jwt.cookie())],
            response::Json(json!({ "message": "ログインに成功しました。" })),
        ))
    } else {
        Err(CustomError::UserNotFound)
    }
}

#[axum_macros::debug_handler]
pub async fn logout(_: Claims) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::SET_COOKIE, JWT::clear_cookie())],
        response::Json(json!({ "message": "ログアウト処理が完了しました。" })),
    )
}

fn create_token(user: &UserEntity) -> Result<JWT, CustomError> {
    let claims = Claims {
        user_id: user.id.unwrap(),
        // UTCタイムスタンプによる有効期限（必須
        exp: 2000000000, // May 2033
    };

    // 認証トークンを作成する。
    // Header: 基本的なJWTヘッダーで、
    // algはデフォルトでHS256、typは自動的にJWTに設定されます。
    // 他のフィールドはすべてオプションです。
    // encode: 与えられたヘッダーとクレームをエンコードし、
    // ヘッダーのアルゴリズムと鍵を用いてペイロードに署名する。
    // 与えられたアルゴリズムがRSAまたはECの場合、鍵はPEM形式である必要がある。
    match encode(&Header::default(), &claims, &KEYS.encoding) {
        Ok(encoded_token) => Ok(JWT::new(&encoded_token)),
        Err(err) => Err(CustomError::CannotEncodeToken(err)),
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignInPayload {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignUpPayload {
    email: String,
    password: String,
    name: String,
}
