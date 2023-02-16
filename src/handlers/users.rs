use std::sync::Arc;

use axum::{
    extract::{self, State},
    http::{header, StatusCode},
    response::{self, IntoResponse},
};
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use validator::Validate;

use crate::{
    entities::user::User as UserEntity, errors::CustomError, jwt::JWT,
    repositories::user::UserRepository, request::Claims, views::user::User, AppState, KEYS,
};

#[axum_macros::debug_handler]
pub async fn get_user(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if let Some(user) = state.user_repository.find_by_user_id(claims.user_id).await {
        Ok((StatusCode::OK, response::Json::<User>(user.into())))
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
    if payload.email.is_empty() || payload.password.is_empty() || payload.display_name.is_empty() {
        return Err(CustomError::MissingCredentials);
    }

    let new_user = UserEntity::create(&payload.email, &payload.password, &payload.display_name);
    // TODO: 既に登録済みのEmailかチェックする
    let saved_user = state.user_repository.store(&new_user).await; // 新規ユーザーをDBに保存

    let jwt = create_token(&saved_user)?;

    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, jwt.cookie())],
        "アカウント新規作成 & ログイン成功",
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
            "アカウント新規作成 & ログイン成功",
        ))
    } else {
        // INTERNAL SERVER ERROR
        unimplemented!()
    }
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
    display_name: String,
}

// #[derive(Debug, Serialize)]
// pub struct AuthBody {
//     access_token: String,
//     token_type: String,
// }

// impl AuthBody {
//     pub fn new(access_token: String) -> Self {
//         Self {
//             access_token,
//             token_type: "Bearer".to_string(),
//         }
//     }
// }

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
