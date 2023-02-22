use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub user_id: Option<String>,
    pub name: String,
    pub avatar_file_name: Option<String>,
    pub email: String,
    pub hashed_password: String,
}

impl User {
    // DBから取得したRowをUserエンティティに変換する用
    // pub fn new(id: i32, email: String, hashed_password: String, name: String) -> User {
    //     User {
    //         id: Some(id),
    //         email,
    //         hashed_password,
    //         name,
    //     }
    // }

    // DBに挿入するためにUserエンティティを作成する用
    pub fn create(email: &str, password: &str, name: &str) -> User {
        User {
            id: None,
            user_id: None,
            name: name.to_string(),
            avatar_file_name: None,
            email: email.to_string(),
            hashed_password: to_sha256(password),
        }
    }

    // pub fn id(&self) -> Option<i32> {
    //     self.id
    // }

    /// ハッシュ化済みのパスワードと平文のパスワードを比較する
    pub fn matches_password(&self, password: &str) -> bool {
        self.hashed_password == to_sha256(password)
    }
}

fn to_sha256(str: &str) -> String {
    let str = str.as_bytes();
    let hashed_str = Sha256::digest(str);
    format!("{:x}", hashed_str)
}
