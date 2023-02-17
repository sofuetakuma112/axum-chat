use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entities::room_members::RoomMember,
    errors::CustomError,
    repositories::{room::RoomRepository, room_member::RoomMemberRepository, user::UserRepository},
    request::Claims,
    views::user::User,
    AppState,
};

/// ルーム一覧を取得する
#[axum_macros::debug_handler]
pub async fn get_rooms(
    claims: Claims,
    Path(user_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    let rooms = state.room_repository.find_by_member_id(user_id).await;

    Ok((StatusCode::OK, response::Json(json!({ "rooms": rooms }))))
}

#[axum_macros::debug_handler]
pub async fn get_room_members(
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
        // member_idからUserを取得する
        let member_ids = room_members
            .iter()
            .map(|x| x.member_id)
            .collect::<Vec<i32>>();
        let users = state.user_repository.find(&member_ids).await;

        Ok((
            StatusCode::OK,
            response::Json(json!({ "room": {
                "room_id": room_id,
                "members": users.into_iter().map(|user| user.into()).collect::<Vec<User>>()
            } })),
        ))
    } else {
        Err(CustomError::AccessingUnauthorisedResources)
    }
}

#[axum_macros::debug_handler]
pub async fn create_room(
    claims: Claims,
    Path(user_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    extract::Json(mut payload): extract::Json<CreateRoomPayload>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    // リクエストを送信したユーザーのuser_idも追加する
    let mut new_member_ids = vec![claims.user_id];
    new_member_ids.append(&mut payload.user_ids);

    let mut room_members = new_member_ids
        .into_iter()
        .map(|user_id| RoomMember::create(None, user_id))
        .collect();
    state.room_member_repository.store(&mut room_members).await;
    Ok((
        StatusCode::CREATED,
        response::Json(json!({ "message": "ルームを新規作成しました。" })),
    ))
}

/// room_idのルームにユーザーを追加する
#[axum_macros::debug_handler]
pub async fn add_room_member(
    claims: Claims,
    Path((user_id, room_id)): Path<(i32, i32)>,
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<CreateRoomMemberPayload>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    // リクエストを送信したユーザーがroom_idのルームのメンバーかチェックする
    let room_members = state.room_member_repository.find_members(room_id).await;
    let is_room_member = room_members.iter().any(|x| x.member_id == claims.user_id);

    if is_room_member {
        let mut room_members = payload
            .user_ids
            .into_iter()
            .map(|user_id| RoomMember::create(Some(room_id), user_id))
            .collect::<Vec<RoomMember>>();
        state.room_member_repository.store(&mut room_members).await;

        Ok((
            StatusCode::CREATED,
            response::Json(json!({ "message": "ルームメンバーを追加しました。" })),
        ))
    } else {
        Err(CustomError::AccessingUnauthorisedResources)
    }
}

#[axum_macros::debug_handler]
pub async fn delete_room_member(
    claims: Claims,
    Path((user_id, room_id, member_id)): Path<(i32, i32, i32)>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    // リクエストを送信したユーザーがroom_idのルームのメンバーかチェックする
    let room_members = state.room_member_repository.find_members(room_id).await;
    let is_room_member = room_members.iter().any(|x| x.member_id == claims.user_id);

    if is_room_member {
        // room_idのルームからmember_idのユーザーを削除する
        let room_member = RoomMember::create(Some(room_id), member_id);
        state.room_member_repository.delete(&room_member).await;

        Ok((
            StatusCode::NO_CONTENT,
            response::Json(json!({ "message": "ルームメンバーを削除しました。" })),
        ))
    } else {
        Err(CustomError::AccessingUnauthorisedResources)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomPayload {
    user_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomMemberPayload {
    user_ids: Vec<i32>,
}
