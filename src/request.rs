use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

use crate::{errors::CustomError, KEYS};

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
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| CustomError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| CustomError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
