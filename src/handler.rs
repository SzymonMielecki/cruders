use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    db::{delete_user, get_all_users, get_single_user, patch_user, post_user, put_user},
    model::{Db, OutUser, PatchUserSchema, StripedUser, User},
};

pub async fn get_user_all_handler(State(db): State<Db>) -> impl IntoResponse {
    let users_res = get_all_users(&db).await;

    match users_res {
        Ok(users) => Json(
            users
                .into_iter()
                .map(|u| u.into())
                .collect::<Vec<OutUser>>(),
        )
        .into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn get_user_single_handler(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = get_single_user(&db, id).await;

    match user {
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
        Ok(user) => Json(OutUser::from(user)).into_response(),
    }
}

pub async fn post_user_handler(
    State(db): State<Db>,
    Json(body): Json<StripedUser>,
) -> impl IntoResponse {
    if body.group != "user" && body.group != "admin" && body.group != "premium" {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let record = User {
        id: None,
        name: body.name,
        lastname: body.lastname,
        birthyear: body.birthyear,
        group: body.group,
    };

    let res = post_user(&db, record).await;

    match res {
        Ok(id) => (StatusCode::CREATED, id).into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn patch_user_handler(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(body): Json<PatchUserSchema>,
) -> impl IntoResponse {
    let res = patch_user(&db, id, body).await;

    match res {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn put_user_handler(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(body): Json<StripedUser>,
) -> impl IntoResponse {
    let res = put_user(&db, id, body).await;

    match res {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn delete_user_handler(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let res = delete_user(&db, id).await;

    match res {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
