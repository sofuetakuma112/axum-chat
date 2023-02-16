use serde::Serialize;

use crate::entities::user::User as UserEntity;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
}

// TweetエンティティからTweetビューに変換するための実装
impl From<UserEntity> for User {
    fn from(user_entity: UserEntity) -> Self {
        User {
            id: user_entity.id.unwrap(),
            display_name: user_entity.display_name,
            email: user_entity.email,
        }
    }
}
