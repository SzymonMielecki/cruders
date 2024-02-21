use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::logic::AppState;

use super::model::{Db, PatchUserSchema, StripedUser, User};

pub async fn get_user_all_handler(State(state): State<AppState>) -> impl IntoResponse {
    let users_res = state.get_all_users_logic().await;

    match users_res {
        Ok(users) => Json(users).into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn get_user_single_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = state.get_single_user_logic(id).await;

    match user {
        Ok(user) => Json(user).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Id not found").into_response(),
    }
}

pub async fn post_user_handler(
    State(state): State<AppState>,
    Json(body): Json<StripedUser>,
) -> impl IntoResponse {
    let res = state.post_user_logic(body).await;

    match res {
        Ok(id) => (StatusCode::CREATED, id).into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn patch_user_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<PatchUserSchema>,
) -> impl IntoResponse {
    let res = state.patch_user_logic(id, body).await;
    match res {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn put_user_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<StripedUser>,
) -> impl IntoResponse {
    let res = state.put_user_logic(id, body).await;
    match res {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let res = state.delete_user_logic(id).await;

    match res {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
