use crate::{errors::CustomError, KEYS};
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::Cookie,
    http::request::Parts,
    RequestPartsExt,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

/// ハンドラ関数の引数に指定するとミドルウェアとして振る舞う？
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}

// リクエストの検証を行いたい場合には FromRequestトレイトを実装します。
// FromRequestトレイトを実装した構造体をハンドラ関数の引数に渡すと、ミドルウェアとして動作する？
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // クッキーからトークンを抽出する
        let TypedHeader(cookies) = parts
            .extract::<TypedHeader<Cookie>>()
            .await
            .map_err(|_| CustomError::InvalidToken)?;
        let jwt = cookies.get("token").ok_or(CustomError::NoToken)?;
        // ユーザーデータのデコード
        let token_data = decode::<Claims>(jwt, &KEYS.decoding, &Validation::default())
            .map_err(|_| CustomError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
