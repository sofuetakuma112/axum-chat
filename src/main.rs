use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use repositories::{
    follow::FollowRepositoryForDb, room::RoomRepositoryForDb,
    room_member::RoomMemberRepositoryForDb,
};

use crate::{
    constants::database_url,
    handlers::{auth, rooms, users},
    repositories::user::UserRepositoryForDb,
};

mod constants;
mod entities;
mod errors;
mod handlers;
mod request;
// mod validator;
mod jwt;
mod repositories;
mod views;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct AppState {
    /// WSのroom、keyはroom名。
    // txs: Mutex<WsRooms>,
    user_repository: UserRepositoryForDb,
    follow_repository: FollowRepositoryForDb,
    room_repository: RoomRepositoryForDb,
    room_member_repository: RoomMemberRepositoryForDb,
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();

    let database_url = database_url();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Could not apply migrations on the database");

    let shared_state = Arc::new(AppState {
        // txs: Mutex::new(WsRooms::default()),
        user_repository: UserRepositoryForDb::new(pool.clone()),
        follow_repository: FollowRepositoryForDb::new(pool.clone()),
        room_repository: RoomRepositoryForDb::new(pool.clone()),
        room_member_repository: RoomMemberRepositoryForDb::new(pool.clone()),
    });

    let app = Router::new()
        .route("/api/auth/user", get(auth::get_user))
        .route("/api/auth/signup", post(auth::signup))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", get(auth::logout))
        .route("/api/users/:user_id/followers", get(users::get_followers))
        .route("/api/users/:user_id/followees", get(users::get_followees))
        .route("/api/users/:user_id/relationships", get(users::follow)) // user_idのユーザーをフォローする
        .route("/api/users/:user_id/relationships", delete(users::unfollow)) // user_idのユーザーをアンフォローする
        .route("/api/users/:user_id/rooms", get(rooms::get_rooms)) // ルームの一覧を取得
        .route("/api/users/:user_id/rooms", post(rooms::create_room)) // ルームの新規作成(追加するユーザーのIDをbodyに持つ)
        .route("/api/users/:user_id/rooms/:room_id/members", get(rooms::get_room_members)) // room_idのルームのメンバー一覧を取得する
        .route("/api/users/:user_id/rooms/:room_id/members", post(rooms::add_room_member)) // room_idのルームに新しいメンバーの追加
        .route(
            "/api/rooms/:room_id/members/:member_id",
            delete(rooms::delete_room_member),
        ) // room_idのルームからmember_idのユーザーを削除する
        .with_state(shared_state); // 受信するすべてのリクエストのExtensionにオブジェクトを挿入するミドルウェアを追加

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

// use axum::{
//     extract::{
//         ws::{Message, WebSocket, WebSocketUpgrade},
//         State,
//     },
//     response::{IntoResponse},
//     routing::get,
//     Router,
// };
// use futures::{sink::SinkExt, stream::StreamExt};
// use serde_json::json;
// use std::{
//     collections::HashSet,
//     net::SocketAddr,
//     sync::{Arc, Mutex},
// };
// use tokio::sync::broadcast;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// // Our shared state
// struct AppState {
//     // 一意のユーザー名が必要です。これにより、どのユーザー名が使用されたかを追跡することができます。
//     user_set: Mutex<HashSet<String>>,
//     // クライアントの接続によって生成したタスク全てに通知を送信するためのチャネル。
//     tx: broadcast::Sender<String>,
// }

// #[tokio::main]
// async fn main() {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "example_chat=trace".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     // with_state()で使用するためにアプリケーションの状態を設定します。
//     let user_set = Mutex::new(HashSet::new());
//     // tx.sendはユーザーに直接データを送信しているわけではない
//     let (tx, _rx) = broadcast::channel(100); // ユーザーごとに生成したタスクのすべてのReceiverハンドルに通知され、その値を受信する

//     let app_state = Arc::new(AppState { user_set, tx });

//     let app = Router::new()
//         .route("/websocket", get(websocket_handler))
//         .with_state(app_state);

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     tracing::debug!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn websocket_handler(
//     ws: WebSocketUpgrade,
//     State(state): State<Arc<AppState>>,
// ) -> impl IntoResponse {
//     ws.on_upgrade(|socket| websocket(socket, state))
// }

// // この関数は、単一のウェブソケット接続、すなわち、単一の接続されたクライアント/ユーザを処理し、そのために2つの独立したタスク（チャットメッセージを受信/送信するため）を生成します。
// async fn websocket(stream: WebSocket, state: Arc<AppState>) {
//     // 分割することで、送信と受信を同時に行う
//     let (mut sender, mut receiver) = stream.split();

//     // ユーザーネームが有効であれば、受信ループで設定されます。
//     let mut username = String::new();
//     // テキストメッセージが見つかるまでループする。
//     while let Some(Ok(message)) = receiver.next().await {
//         if let Message::Text(name) = message {
//             // クライアントから送信されたユーザー名が使用されていない場合、ユーザー名文字列を記入する。
//             username = check_username(&state, &name);

//             // 空でなければループを終了させ、そうでなければ関数を終了させます。
//             if !username.is_empty() {
//                 break;
//             } else {
//                 // ユーザー名が使われているものだけをクライアントに送信してください。
//                 sender
//                     .send(Message::Text(String::from("Username already taken.")))
//                     .await
//                     .ok();

//                 return;
//             }
//         }
//     }

//     // "joined"メッセージを送信する前にサブスクライブすることで、クライアントにも表示させる。
//     let mut rx = state.tx.subscribe(); // このsubscribeの呼び出しの後に送信された値を受け取る新しいReceiverハンドルを作成します。

//     let msg =
//         json!({ "message": format!("{} が参加しました。", username), "name": "BOT" }).to_string();
//     tracing::debug!("{}", msg);
//     state.tx.send(msg).ok(); // すべての購読者に「joined」メッセージを送信

//     // ブロードキャストメッセージを受信し、ウェブソケット経由でテキストメッセージをクライアントに送信する最初のタスクを起動します。
//     let mut send_task = tokio::spawn(async move {
//         while let Ok(msg) = rx.recv().await {
//             // tx.sendをここで受け取る？
//             if sender.send(Message::Text(msg)).await.is_err() {
//                 // 1人に向けてメッセージを送信
//                 break;
//             }
//         }
//     });

//     // 受け手のタスクに渡したい（移動させたい）ものをクローンする。
//     let tx = state.tx.clone();
//     let name = username.clone();

//     // ウェブソケットからメッセージを受け取り、ユーザー名を前置きして、すべてのブロードキャスト購読者に送信するタスクを起動します。
//     let mut recv_task = tokio::spawn(async move {
//         while let Some(Ok(Message::Text(text))) = receiver.next().await {
//             // 1対1通信用のReceiverからメッセージを受信
//             // メッセージの前にユーザー名を追加する。
//             let _ = tx.send(json!({ "message": text, "name": name }).to_string());
//             // ブロードキャストのSenderを通じてすべての購読者に送信する
//         }
//     });

//     // どちらかのタスクが完了まで実行されたら、もう一方を中止する。
//     tokio::select! {
//         _ = (&mut send_task) => recv_task.abort(),
//         _ = (&mut recv_task) => send_task.abort(),
//     };

//     // "user left"メッセージを送信する（上記の "joined"と同様）
//     let msg =
//         json!({ "message": format!("{} が退出しました。", username), "name": "BOT" }).to_string();
//     tracing::debug!("{}", msg);
//     let _ = state.tx.send(msg); // すべての購読者にメッセージを送信

//     // ユーザー名をMapから削除し、新しいクライアントが再び利用できるようにする。
//     state.user_set.lock().unwrap().remove(&username);
// }

// fn check_username(state: &AppState, name: &str) -> String {
//     let mut user_set = state.user_set.lock().unwrap();

//     if user_set.contains(name) {
//         String::new()
//     } else {
//         user_set.insert(name.to_owned());
//         name.to_string()
//     }
// }
