use std::sync::Arc;

use crate::{request::Claims, AppState};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

// ハンドラ関数の戻り値はIntoResponseトレイトを実装している必要がある
pub fn sample(
    claims: Claims, // 認証が必要なリクエストのハンドラー関数の引数にUserContext型の引数を追加するだけで、自動的にミドルウェアが割り当てられる？
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Html("<h1>Hello, World!</h1>")
}

// ログインページを返す
// async fn login(query: Query<LoginQuery>) -> impl IntoResponse {
//     let empty_session_token = services::clear_session();
//     let headers = Headers(vec![("Set-Cookie", empty_session_token.cookie())]);
//     let response = response::from_template(SignIn {
//         error: query.error.is_some(),
//     });
//     (headers, response)
// }

// サインアップページを返す
// async fn register() -> impl IntoResponse {
//     response::from_template(SignUp)
// }

// #[derive(Deserialize)]
// struct LoginQuery {
//     error: Option<String>,
// }
