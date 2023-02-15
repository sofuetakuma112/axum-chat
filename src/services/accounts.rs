use sqlx::{Pool, Postgres};

use crate::request::Claims;
use crate::{entities::account::Account};
use crate::stores::accounts;
use crate::KEYS;
use jsonwebtoken::{encode, Header};

pub async fn create_account(pool: Pool<Postgres>, email: &str, password: &str, display_name: &str) {
    let new_account = Account::create(email, password, display_name);
    accounts::store(pool, &new_account).await;
}

/// 引数のemailを使ってDBからアカウントを取得して、引数のpasswordと照合し一致した場合、セッションをDBに格納してクッキー文字列を返す
pub async fn create_token(pool: Pool<Postgres>, email: &str, password: &str) -> Option<String> {
    if let Some(account) = accounts::find_by(pool, email).await {
        if !account.matches_password(password) {
            return None;
        }

        let claims = Claims {
            user_id: account.id.unwrap(),
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
            Ok(token) => Some(token),
            Err(_) => None,
        }
    } else {
        None
    }
}

// pub fn clear_session() -> SessionToken {
//     SessionToken::clear()
// }

// pub struct SessionToken {
//     token: String,
//     max_age: usize,
// }

// impl SessionToken {
//     pub fn new(token: &str) -> SessionToken {
//         SessionToken {
//             token: token.to_string(),
//             max_age: 604800,
//         }
//     }

//     pub fn clear() -> SessionToken {
//         SessionToken {
//             token: "deleted".to_string(),
//             max_age: 0,
//         }
//     }
// }

// impl SessionToken {
//     pub fn cookie(&self) -> String {
//         format!(
//             "{}={}; Max-Age={}; Path=/; HttpOnly",
//             AXUM_SESSION_COOKIE_NAME, self.token, self.max_age
//         )
//     }
// }
