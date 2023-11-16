use serde_json::json;

use crate::test_helper::{
    patch_name_from_full, record_1, record_1_patched, record_2, stripped_from_full,
    test_db_empty_raw, test_db_pushed_raw, test_db_raw, test_server,
};

#[tokio::test]
async fn test_get_all_no_modify_good() {
    let (server, _) = test_server();
    let response = server.get("/users").await;

    response.assert_json(&json!(test_db_raw()))
}

#[tokio::test]
async fn test_get_all_post_good() {
    let (server, _) = test_server();
    server
        .post("/users")
        .json(&json!(stripped_from_full(record_2())))
        .await;
    let response = server.get("/users").await;

    response.assert_json(&json!(test_db_pushed_raw()))
}

#[tokio::test]
async fn test_get_single_no_modify_good() {
    let (server, _) = test_server();
    let response = server.get("/users/1").await;

    response.assert_json(&json!(stripped_from_full(record_1())))
}

#[tokio::test]
async fn test_get_single_post_good() {
    let (server, _) = test_server();
    server
        .post("/users")
        .json(&json!(stripped_from_full(record_2())))
        .await;
    let response = server.get("/users/2").await;

    response.assert_json(&json!(stripped_from_full(record_2())))
}

#[tokio::test]
async fn test_get_single_patch_good() {
    let (server, _) = test_server();
    server
        .patch("/users/1")
        .json(&json!(patch_name_from_full(record_2())))
        .await;
    let response = server.get("/users/1").await;

    response.assert_json(&json!(stripped_from_full(record_1_patched())))
}

#[tokio::test]
async fn test_get_single_put_modify_good() {
    let (server, _) = test_server();
    server
        .put("/users/1")
        .json(&json!(stripped_from_full(record_2())))
        .await;
    let response = server.get("/users/1").await;

    response.assert_json(&json!(stripped_from_full(record_2())))
}

#[tokio::test]
async fn test_get_single_put_new_good() {
    let (server, _) = test_server();
    server
        .put("/users/2")
        .json(&json!(stripped_from_full(record_2())))
        .await;
    let response = server.get("/users/2").await;

    response.assert_json(&json!(stripped_from_full(record_2())))
}

#[tokio::test]
async fn test_get_single_delete_good() {
    let (server, _) = test_server();
    server.delete("/users/1").await;
    let response = server.get("/users").await;

    response.assert_json(&json!(test_db_empty_raw()))
}
