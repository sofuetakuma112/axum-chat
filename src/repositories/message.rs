use axum::async_trait;
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
        sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE room_id = $1 ORDER BY created_at DESC")
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
}

#[async_trait]
pub trait MessageRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find_by_room_id(&self, member_id: i32) -> Vec<Message>;
    async fn store(&self, message: &Message) -> Message;
}
