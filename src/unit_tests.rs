use axum::http::StatusCode;

use serde_json::json;

use crate::test_helper::{
    bad_json, patch_name_from_full, record_2, stripped_from_full, test_server,
};

#[tokio::test]
async fn test_endpoint_get_all_users_good() {
    let (server, _) = test_server();
    let response = server.get("/users").await;

    response.assert_status_ok()
}

#[tokio::test]
async fn test_endpoint_get_single_user_good() {
    let (server, _) = test_server();
    let response = server.get("/users/1").await;

    response.assert_status_ok()
}

#[tokio::test]
async fn test_endpoint_get_single_user_bad() {
    let (server, _) = test_server();
    let response = server.get("/users/2").await;

    response.assert_status(StatusCode::NO_CONTENT)
}

// TODO zapytac sie o to czy tyle wystarczy (czy uzyc tez geta zeby sprawdzic baze)
#[tokio::test]
async fn test_endpoint_post_user_good() {
    let (server, _) = test_server();
    let response = server.post("/users").json(&json!(record_2())).await;

    response.assert_status(StatusCode::CREATED)
}

#[tokio::test]
async fn test_endpoint_post_user_bad() {
    let (server, _) = test_server();
    let response = server.post("/users").json(&json!(bad_json())).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY)
}

#[tokio::test]
async fn test_endpoint_patch_user_good() {
    let (server, _) = test_server();
    let response = server
        .patch("/users/1")
        .json(&json!(patch_name_from_full(record_2())))
        .await;
    response.assert_status(StatusCode::NO_CONTENT)
}
#[tokio::test]
async fn test_endpoint_patch_user_bad() {
    let (server, _) = test_server();
    let response = server.patch("/users/1").json(&json!(bad_json())).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY)
}

#[tokio::test]
async fn test_endpoint_put_user_good() {
    let (server, _) = test_server();
    let response = server
        .put("/users/1")
        .json(&json!(stripped_from_full(record_2())))
        .await;

    response.assert_status(StatusCode::NO_CONTENT)
}

#[tokio::test]
async fn test_endpoint_put_user_bad() {
    let (server, _) = test_server();
    let response = server.put("/users/1").json(&json!(bad_json())).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY)
}

#[tokio::test]
async fn test_endpoint_delete_user_good() {
    let (server, _) = test_server();
    let response = server.delete("/users/1").await;

    response.assert_status(StatusCode::NO_CONTENT)
}

#[tokio::test]
async fn test_endpoint_delete_user_bad() {
    let (server, _) = test_server();
    let response = server.delete("/users/2").await;

    response.assert_status(StatusCode::BAD_REQUEST)
}
