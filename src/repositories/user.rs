use axum::async_trait;
use sqlx::PgPool;

use crate::entities::users::User;

#[derive(Debug, Clone)]
pub struct UserRepositoryForDb {
    pool: PgPool,
}

impl UserRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        UserRepositoryForDb { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryForDb {
    async fn find(&self, ids: &Vec<i32>) -> Vec<User> {
        if ids.is_empty() {
            return Vec::new();
        }

        let ids_str = ids
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        sqlx::query_as::<_, User>(&format!("SELECT * FROM users WHERE id in ({})", ids_str))
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    /// user_idを使ってusersからユーザーを検索する
    async fn find_by_user_id(&self, user_id: i32) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    /// DBにアカウントレコードを新規追加
    async fn store(&self, entity: &User) -> User {
        sqlx::query_as::<_, User>("INSERT INTO users (email, hashed_password, display_name) VALUES ($1, $2, $3) RETURNING id, email, hashed_password, display_name")
            .bind(&entity.email)
            .bind(&entity.hashed_password)
            .bind(&entity.display_name)
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }
}

#[async_trait]
pub trait UserRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find(&self, ids: &Vec<i32>) -> Vec<User>;
    async fn find_by_user_id(&self, user_id: i32) -> Option<User>;
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn store(&self, entity: &User) -> User;
}

// // todosテーブルのみ
// #[derive(Debug, Clone, PartialEq, Eq, FromRow)]
// struct UserFromRow {
//     id: i32,
//     text: String,
//     completed: bool,
// }

// // OUTER JOIN
// #[derive(Debug, Clone, PartialEq, Eq, FromRow)]
// struct UserWithLabelFromRow {
//     id: i32,
//     text: String,
//     completed: bool,
//     label_id: Option<i32>,
//     label_name: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct UserEntity {
//     pub id: i32,
//     pub text: String,
//     pub completed: bool,
//     pub labels: Vec<Label>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
// pub struct CreateUser {
//     #[validate(length(min = 1, message = "Can not be empty"))]
//     #[validate(length(max = 100, message = "Over text length"))]
//     text: String,
//     labels: Vec<i32>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
// pub struct UpdateUser {
//     #[validate(length(min = 1, message = "Can not be empty"))]
//     #[validate(length(max = 100, message = "Over text length"))]
//     text: Option<String>,
//     completed: Option<bool>,
//     labels: Option<Vec<i32>>,
// }
