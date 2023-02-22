use std::sync::Arc;

use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    errors::CustomError, repositories::{follow::FollowRepository, user::UserRepository}, request::Claims,
    views::user::User, AppState,
};

#[axum_macros::debug_handler]
pub async fn get_users(
    _claims: Claims,
    Query(payload): Query<GetUsersPayload>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    let users = state.user_repository.find_by_user_id(payload.user_id.as_str()).await;
    Ok((
        StatusCode::OK,
        response::Json(
            json!({ "users": users.into_iter().map(|user| User::from(user)).collect::<Vec<User>>() }),
        ),
    ))
}

/// user_idのユーザーをフォローしているユーザー一覧を返す
#[axum_macros::debug_handler]
pub async fn get_followers(
    claims: Claims,
    Path(user_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    let followers = state.follow_repository.find_follower(user_id).await;
    Ok((
        StatusCode::OK,
        response::Json(
            json!({ "followers": followers.into_iter().map(|follow| User::from(follow)).collect::<Vec<User>>() }),
        ),
    ))
}

/// user_idのユーザーがフォローしているユーザー一覧を返す
#[axum_macros::debug_handler]
pub async fn get_followees(
    claims: Claims,
    Path(user_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    if user_id != claims.user_id {
        return Err(CustomError::AccessingUnauthorisedResources);
    }

    let followees = state.follow_repository.find_followee(user_id).await;
    Ok((
        StatusCode::OK,
        response::Json(
            json!({ "followees": followees.into_iter().map(|follow| User::from(follow)).collect::<Vec<User>>() }),
        ),
    ))
}

#[axum_macros::debug_handler]
pub async fn follow(
    claims: Claims,
    Path(user_id): Path<i32>, // フォローするユーザーID
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    state
        .follow_repository
        .follow(claims.user_id, user_id)
        .await;
    Ok((
        StatusCode::CREATED,
        response::Json(json!({ "message": "フォローしました。" })),
    ))
}

#[axum_macros::debug_handler]
pub async fn unfollow(
    claims: Claims,
    Path(user_id): Path<i32>, // フォローするユーザーID
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, CustomError> {
    state
        .follow_repository
        .unfollow(claims.user_id, user_id)
        .await;
    Ok((
        StatusCode::NO_CONTENT,
        response::Json(json!({ "message": "フォローを解除しました。" })),
    ))
}

#[derive(Deserialize)]
pub struct GetUsersPayload {
    user_id: String
}
