use axum::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::entities::{room_members::RoomMember, rooms::Room};

#[derive(Debug, Clone)]
pub struct RoomMemberRepositoryForDb {
    pool: PgPool,
}

impl RoomMemberRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        RoomMemberRepositoryForDb { pool }
    }
}

#[async_trait]
impl RoomMemberRepository for RoomMemberRepositoryForDb {
    async fn find_members(&self, room_id: i32) -> Vec<RoomMember> {
        sqlx::query_as::<_, RoomMember>("SELECT * FROM room_members WHERE room_id = $1")
            .bind(&room_id)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    async fn delete(&self, room_member: &RoomMember) {
        sqlx::query("DELETE FROM room_members WHERE room_id = $1 AND member_id = $2")
            .bind(&room_member.room_id)
            .bind(&room_member.member_id)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    // RoomMemberの登録
    async fn store(&self, room_members: &mut Vec<RoomMember>) {
        // 一つでもroom_idがNoneのRoomMemberがあればroomsにレコードを追加する
        let no_room_id = room_members.iter().any(|x| x.room_id.is_none());

        if no_room_id {
            // ルームがない場合
            // roomsにレコードを追加する
            let room = sqlx::query_as::<_, Room>("INSERT INTO rooms DEFAULT VALUES RETURNING id")
                .fetch_one(&self.pool)
                .await
                .unwrap();
            // 新規作成したRoomのidを全てのRoomMemberにセットする
            for room_member in room_members.iter_mut() {
                room_member.room_id = room.id;
            }
        }

        // 参照: https://docs.rs/sqlx-core/0.5.13/sqlx_core/query_builder/struct.QueryBuilder.html#method.push_values
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            // ほとんどの `QueryBuilder` は、
            // 識別子や引用符で囲まれた文字列など、
            // 正確な値が必要な場合には自動的にスペースを挿入しません。
            "INSERT INTO room_members(room_id, member_id) ",
        );

        query_builder.push_values(room_members.into_iter(), |mut b, room_member| {
            // 値ではなく、参照でバインドしたい場合は、
            // `query_builder` と同じ時間だけ生きている参照を生成するイテレータが必要で、
            // 例えば、最初に `Vec` に収集します。
            b.push_bind(room_member.room_id.unwrap())
                .push_bind(room_member.member_id);
        });

        let query = query_builder.build();

        query.execute(&self.pool).await.unwrap();
    }
}

#[async_trait]
pub trait RoomMemberRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find_members(&self, room_id: i32) -> Vec<RoomMember>;
    async fn delete(&self, room_member: &RoomMember);
    async fn store(&self, room_member: &mut Vec<RoomMember>);
}
