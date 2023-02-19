use serde::{Serialize, Deserialize};

use crate::entities::{messages::Message as MessageEntity, users::User};

// Messageビュー
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Message {
    pub id: i32,
    pub name: String,
    pub message: String,
    pub posted_at: String,
}

// MessageエンティティからMessageビューに変換するための実装
impl From<(MessageEntity, &User)> for Message {
    fn from(e: (MessageEntity, &User)) -> Self {
        let (message_entity, user_entity) = e;
        Message {
            id: message_entity.id.unwrap_or(-1),
            name: user_entity.display_name.clone(),
            message: message_entity.message,
            posted_at: message_entity
                .created_at
                .unwrap()
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
        }
    }
}
