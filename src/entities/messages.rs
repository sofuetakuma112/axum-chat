#[derive(sqlx::FromRow)]
pub struct Message {
    pub id: Option<i32>,
    pub room_id: i32,
    pub user_id: i32,
    pub message: String,
    pub message_type: MessageType,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::Type)]
#[repr(i32)]
pub enum MessageType {
    Text,
    Image,
}

impl Message {
    // DBから取得したRowをMessageエンティティに変換する用
    // pub fn new(
    //     id: i32,
    //     room_id: i32,
    //     user_id: i32,
    //     message: String,
    //     created_at: chrono::NaiveDateTime,
    // ) -> Message {
    //     Message {
    //         id: Some(id),
    //         room_id,
    //         user_id,
    //         message,
    //         created_at: Some(created_at),
    //     }
    // }

    // DBに挿入するためにMessageエンティティを作成する用
    pub fn create(
        room_id: i32,
        user_id: i32,
        message: String,
        message_type: MessageType,
    ) -> Message {
        Message {
            id: None,
            room_id,
            user_id,
            message,
            message_type,
            created_at: None,
        }
    }
}
