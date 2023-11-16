use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::model::{PatchUserSchema, StripedUser, User, DB};

pub async fn get_user_all_handler(State(db): State<DB>) -> impl IntoResponse {
    let db = db.lock().await;

    Json(db.clone().into_iter().collect::<Vec<User>>())
}

pub async fn get_user_single_handler(
    State(db): State<DB>,
    Path(id_string): Path<String>,
) -> impl IntoResponse {
    let db = db.lock().await;

    let id = match id_string.parse::<u32>() {
        Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
        Ok(id) => id,
    };

    match db.iter().find(|user| user.id == Some(id)) {
        None => StatusCode::BAD_REQUEST.into_response(),
        Some(user) => Json(StripedUser {
            name: user.name.clone(),
            lastname: user.lastname.clone(),
        })
        .into_response(),
    }
}

pub async fn post_user_handler(
    State(db): State<DB>,
    Json(mut body): Json<User>,
) -> impl IntoResponse {
    let mut db = db.lock().await;

    let biggest_id_user_opt = db.iter().max_by_key(|u| u.id);
    let biggest_id = match biggest_id_user_opt {
        Some(biggest_id_user) => biggest_id_user.id.unwrap_or(0),
        None => 0,
    };
    body.id = Some(biggest_id + 1);

    db.push(body);

    StatusCode::CREATED
}

pub async fn patch_user_handler(
    State(db): State<DB>,
    Path(id_string): Path<String>,
    Json(body): Json<PatchUserSchema>,
) -> impl IntoResponse {
    let mut db = db.lock().await;

    let id = match id_string.parse::<u32>() {
        Err(_) => return StatusCode::BAD_GATEWAY,
        Ok(id) => id,
    };

    if let Some(user) = db.iter_mut().find(|user| user.id == Some(id)) {
        let name = body.name.unwrap_or_else(|| user.name.to_owned());
        let lastname = body.lastname.unwrap_or_else(|| user.lastname.to_owned());

        *user = User {
            id: user.id.to_owned(),
            name,
            lastname,
        };

        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn put_user_handler(
    State(db): State<DB>,
    Path(id_string): Path<String>,
    Json(body): Json<StripedUser>,
) -> impl IntoResponse {
    let mut db = db.lock().await;

    let id = match id_string.parse::<u32>() {
        Err(_) => return StatusCode::BAD_GATEWAY,
        Ok(id) => id,
    };

    let payload = User {
        id: Some(id),
        name: body.name,
        lastname: body.lastname,
    };

    if let Some(user) = db.iter_mut().find(|user| user.id == Some(id)) {
        *user = payload;
    } else {
        db.push(payload);
    }
    StatusCode::OK
}

pub async fn delete_user_handler(
    State(db): State<DB>,
    Path(id_string): Path<String>,
) -> impl IntoResponse {
    let mut db = db.lock().await;

    let id = match id_string.parse::<u32>() {
        Err(_) => return StatusCode::BAD_GATEWAY,
        Ok(id) => id,
    };

    if let Some(pos) = db.iter().position(|user| user.id == Some(id)) {
        db.remove(pos);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
