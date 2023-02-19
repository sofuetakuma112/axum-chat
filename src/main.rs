use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue},
    routing::{delete, get, get_service, post},
    Router,
};
use handlers::ws::WsRooms;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use repositories::{
    follow::FollowRepositoryForDb, message::MessageRepositoryForDb, room::RoomRepositoryForDb,
    room_member::RoomMemberRepositoryForDb,
};
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};
use http::Method;

use crate::{
    constants::database_url,
    errors::CustomError,
    handlers::{auth, messages, rooms, users, ws},
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
mod services;
mod views;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct AppState {
    /// WSのroom、keyはroom名。
    txs: Mutex<WsRooms>,
    user_repository: UserRepositoryForDb,
    follow_repository: FollowRepositoryForDb,
    room_repository: RoomRepositoryForDb,
    room_member_repository: RoomMemberRepositoryForDb,
    message_repository: MessageRepositoryForDb,
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
        txs: Mutex::new(WsRooms::default()),
        user_repository: UserRepositoryForDb::new(pool.clone()),
        follow_repository: FollowRepositoryForDb::new(pool.clone()),
        room_repository: RoomRepositoryForDb::new(pool.clone()),
        room_member_repository: RoomMemberRepositoryForDb::new(pool.clone()),
        message_repository: MessageRepositoryForDb::new(pool.clone()),
    });

    let app = Router::new()
        .route("/api/auth/user", get(auth::get_user))
        .route("/api/auth/signup", post(auth::signup))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", delete(auth::logout))
        .route("/api/users/:user_id/followers", get(users::get_followers))
        .route("/api/users/:user_id/followees", get(users::get_followees))
        .route("/api/users/:user_id/relationships", get(users::follow)) // user_idのユーザーをフォローする
        .route("/api/users/:user_id/relationships", delete(users::unfollow)) // user_idのユーザーをアンフォローする
        .route("/api/users/:user_id/rooms", get(rooms::get_rooms)) // ルームの一覧を取得
        .route("/api/users/:user_id/rooms", post(rooms::create_room)) // ルームの新規作成(追加するユーザーのIDをbodyに持つ)
        .route(
            "/api/users/:user_id/rooms/:room_id/members",
            get(rooms::get_room_members),
        ) // room_idのルームのメンバー一覧を取得する
        .route(
            "/api/users/:user_id/rooms/:room_id/members",
            post(rooms::add_room_member),
        ) // room_idのルームに新しいメンバーの追加
        .route(
            "/api/rooms/:room_id/members/:member_id",
            delete(rooms::delete_room_member),
        ) // room_idのルームからmember_idのユーザーを削除する
        .route(
            "/api/users/:user_id/rooms/:room_id/messages",
            get(messages::get_messages),
        )
        .route(
            "/api/users/:user_id/rooms/:room_id/messages",
            post(messages::create_message),
        )
        .route("/ws", get(ws::ws_test_handler))
        .route("/ws/users/:user_id/rooms", get(ws::rooms_handler))
        .route(
            "/ws/users/:user_id/rooms/:room_id",
            get(ws::messages_handler),
        )
        .nest_service(
            "/static",
            get_service(ServeDir::new("static"))
                .handle_error(|_| async { CustomError::FileNotFound }),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
                .allow_headers(vec![CONTENT_TYPE])
                .allow_credentials(true),
        )
        .with_state(shared_state); // 受信するすべてのリクエストのExtensionにオブジェクトを挿入するミドルウェアを追加

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
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
