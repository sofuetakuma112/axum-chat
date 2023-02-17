use axum::async_trait;
use sqlx::PgPool;

use crate::entities::rooms::Room;

#[derive(Debug, Clone)]
pub struct RoomRepositoryForDb {
    pool: PgPool,
}

impl RoomRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        RoomRepositoryForDb { pool }
    }
}

#[async_trait]
impl RoomRepository for RoomRepositoryForDb {
    async fn find_by_member_id(&self, member_id: i32) -> Vec<Room> {
        sqlx::query_as::<_, Room>(
            "SELECT * FROM rooms WHERE id in (
SELECT room_id FROM room_members WHERE member_id = $1
)",
        )
        .bind(&member_id)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    async fn store(&self) -> Room {
        sqlx::query_as::<_, Room>("INSERT INTO rooms RETURNING id")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }
}

#[async_trait]
pub trait RoomRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find_by_member_id(&self, member_id: i32) -> Vec<Room>;
    async fn store(&self) -> Room;
}
