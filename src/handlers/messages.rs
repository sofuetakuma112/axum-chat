use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entities::messages::Message as MessageEntity,
    errors::CustomError,
    repositories::{
        message::MessageRepository, room_member::RoomMemberRepository, user::UserRepository,
    },
    request::Claims,
    services::messages::list_messages,
    AppState,
};

#[axum_macros::debug_handler]
pub async fn get_messages(
    claims: Claims,
    Path((user_id, room_id)): Path<(i32, i32)>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    // リクエストを送信したユーザーがroom_idのルームのメンバーかチェックする
    let room_members = state.room_member_repository.find_members(room_id).await;
    let is_room_member = room_members.iter().any(|x| x.member_id == claims.user_id);

    if is_room_member {
        let messages = list_messages(
            state.message_repository.clone(),
            state.user_repository.clone(),
            room_id,
        )
        .await;

        Ok((
            StatusCode::OK,
            response::Json(json!({ "room": {
                "room_id": room_id,
                "messages": messages
            } })),
        ))
    } else {
        Err(CustomError::AccessingUnauthorisedResources)
    }
}

#[axum_macros::debug_handler]
pub async fn create_message(
    claims: Claims,
    Path((user_id, room_id)): Path<(i32, i32)>,
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<CreateMessagePayload>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    // リクエストを送信したユーザーがroom_idのルームのメンバーかチェックする
    let room_members = state.room_member_repository.find_members(room_id).await;
    let is_room_member = room_members.iter().any(|x| x.member_id == claims.user_id);

    if is_room_member {
        let message = MessageEntity::create(room_id, user_id, payload.message.clone());
        state.message_repository.store(&message).await;

        Ok((
            StatusCode::CREATED,
            response::Json(json!({ "message": "メッセージを新規投稿しました。" })),
        ))
    } else {
        Err(CustomError::AccessingUnauthorisedResources)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMessagePayload {
    message: String,
}
