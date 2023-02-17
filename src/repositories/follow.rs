use axum::async_trait;
use sqlx::PgPool;

use crate::entities::users::User;

#[derive(Debug, Clone)]
pub struct FollowRepositoryForDb {
    pool: PgPool,
}

impl FollowRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        FollowRepositoryForDb { pool }
    }
}

#[async_trait]
impl FollowRepository for FollowRepositoryForDb {
    /// user_idを使って、そのuser_idのユーザーをフォローするユーザーを取得する
    async fn find_follower(&self, user_id: i32) -> Vec<User> {
        // followee_idがuser_idのレコードの全てのfollower_idを取得する
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id in (
SELECT follower_id FROM follows WHERE followee_id = $1
)",
        )
        .bind(&user_id)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    /// 自分がフォローしているユーザーを返す
    async fn find_followee(&self, user_id: i32) -> Vec<User> {
        // followee_idがuser_idのレコードの全てのfollower_idを取得する
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id in (
SELECT followee_id FROM follows WHERE follower_id = $1
)",
        )
        .bind(&user_id)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    async fn follow(&self, follower_id: i32, followee_id: i32) {
        sqlx::query("INSERT INTO follows (follower_id, followee_id) VALUES($1, $2)")
            .bind(&follower_id)
            .bind(&followee_id)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    async fn unfollow(&self, follower_id: i32, followee_id: i32) {
        sqlx::query("DELETE FROM follows WHERE follower_id = $1 AND followee_id = $2")
            .bind(&follower_id)
            .bind(&followee_id)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

#[async_trait]
pub trait FollowRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find_follower(&self, user_id: i32) -> Vec<User>;
    async fn find_followee(&self, user_id: i32) -> Vec<User>;
    async fn follow(&self, follower_id: i32, followee_id: i32);
    async fn unfollow(&self, follower_id: i32, followee_id: i32);
}
