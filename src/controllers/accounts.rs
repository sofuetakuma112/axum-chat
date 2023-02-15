use std::sync::Arc;

use axum::{
    extract::{self, State},
    response,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{errors::AuthError, services, AppState};

// アカウント新規作成
#[axum_macros::debug_handler]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<SignUpPayload>,
) -> Result<response::Json<AuthBody>, AuthError> {
    // ユーザーがクレデンシャルを送信したかどうかを確認する
    if payload.email.is_empty() || payload.password.is_empty() || payload.display_name.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    services::accounts::create_account(
        state.pool.clone(),
        &payload.email,
        &payload.password,
        &payload.display_name,
    )
    .await;

    if let Some(token) =
        services::accounts::create_token(state.pool.clone(), &payload.email, &payload.password)
            .await
    {
        Ok(response::Json(AuthBody::new(token)))
    } else {
        Err(AuthError::WrongCredentials)
    }
}

// ログイン
#[axum_macros::debug_handler]
pub async fn signin(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<SignInPayload>,
) -> Result<response::Json<AuthBody>, AuthError> {
    if let Some(token) =
        services::accounts::create_token(state.pool.clone(), &payload.email, &payload.password)
            .await
    {
        Ok(response::Json(AuthBody::new(token)))
    } else {
        Err(AuthError::WrongCredentials)
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
    display_name: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

// fn redirect_with_session(
//     session: Option<SessionToken>,
// ) -> Result<impl IntoResponse, impl IntoResponse> {
//     if let Some(session_token) = session {
//         let headers = Headers(vec![("Set-Cookie", session_token.cookie())]);
//         let response = Redirect::to(Uri::from_static("/"));
//         // <T: IntoResponse> (Headers, T) 型も IntoResponse を実装しています。
//         Ok((headers, response)) // レスポンスヘッダ付きでリダイレクトさせるときはタプルで書く？
//     } else {
//         Err(Redirect::to(Uri::from_static("/login?error=invalid"))) // ログインページを再度表示
//     }
// }
