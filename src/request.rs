use crate::{errors::CustomError, KEYS};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
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
        let headers = &parts.headers;
        let cookie_jar: CookieJar = CookieJar::from_headers(headers);
        let Some(cookie) = cookie_jar.get("token")  else {
            return Err(CustomError::NoToken);
        };
        match decode::<Claims>(cookie.value(), &KEYS.decoding, &Validation::default()) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err(CustomError::InvalidToken),
        }
    }
}
