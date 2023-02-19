use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct RoomMember {
    pub room_id: Option<i32>,
    pub member_id: i32,
}

impl RoomMember {
    // DBから取得したRowをRoomMemberエンティティに変換する用
    // pub fn new(room_id: i32, member_id: i32) -> RoomMember {
    //     RoomMember {
    //         room_id: Some(room_id),
    //         member_id,
    //     }
    // }

    // DBに挿入するためにRoomMemberエンティティを作成する用
    pub fn create(room_id: Option<i32>, member_id: i32) -> RoomMember {
        RoomMember { room_id, member_id }
    }
}
