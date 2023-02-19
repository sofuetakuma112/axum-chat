use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Room {
    pub id: Option<i32>,
}

// impl Room {
//     // DBから取得したRowをRoomエンティティに変換する用
//     pub fn new(id: i32) -> Room {
//         Room { id: Some(id) }
//     }

//     // DBに挿入するためにRoomエンティティを作成する用
//     pub fn create() -> Room {
//         Room { id: None }
//     }
// }
