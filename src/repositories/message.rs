use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::entities::messages::Message;

#[derive(Debug, Clone)]
pub struct MessageRepositoryForDb {
    pool: PgPool,
}

impl MessageRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        MessageRepositoryForDb { pool }
    }
}

#[async_trait]
impl MessageRepository for MessageRepositoryForDb {
    async fn find_by_room_id(&self, room_id: i32) -> Vec<Message> {
        sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE room_id = $1 ORDER BY created_at DESC",
        )
        .bind(&room_id)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    async fn store(&self, message: &Message) -> Message {
        sqlx::query_as::<_, Message>(
            "INSERT INTO messages (user_id, room_id, message) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&message.user_id)
        .bind(&message.room_id)
        .bind(&message.message)
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    /// 既存のメッセージのリストを見たものとしてマークするか？
    ///
    /// # 引数
    ///
    /// - messages_uuid : 見たメッセージのリスト。
    /// - pool : 接続プール。
    async fn mark_as_seen(
        message_ids: &Vec<i32>,
        pool: &sqlx::PgPool,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE MESSAGE SET reception_status=$1 WHERE uuid = ANY($2)")
            .bind(WsReceptionStatus::Seen)
            .bind(message_ids)
            .execute(pool)
            .await
    }
}

#[async_trait]
pub trait MessageRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find_by_room_id(&self, member_id: i32) -> Vec<Message>;
    async fn store(&self, message: &Message) -> Message;
    async fn mark_as_seen(
        messages_uuid: &Vec<i32>,
        pool: &sqlx::PgPool,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error>;
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    derivative::Derivative,
    PartialEq,
    Eq,
    Hash,
    Default,
    Copy,
    sqlx::Type,
)]
#[repr(i32)]
pub enum WsReceptionStatus {
    #[default]
    NotSent = 1,
    Sent = 2,
    Seen = 3,
}
