use axum::http::StatusCode;
use axum_test::TestServer;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    model::{NamePatch, StripedUser, User},
    route::{create_router, join_router_db},
};

fn test_server() -> (TestServer, Arc<Mutex<Vec<User>>>) {
    let db = test_db_1_record();
    let app = join_router_db(create_router(), &db);

    let server = TestServer::new(app).unwrap();

    (server, db)
}

fn test_db_1_record() -> Arc<Mutex<Vec<User>>> {
    Arc::new(Mutex::new(vec![record_1()]))
}

fn record_1() -> User {
    User {
        id: 1,
        name: String::from("John"),
        lastname: String::from("Doe"),
    }
}

fn stripped_from_full(user: User) -> StripedUser {
    StripedUser {
        name: user.name,
        lastname: user.lastname,
    }
}

fn patch_name_from_full(user: User) -> NamePatch {
    NamePatch { name: user.name }
}

fn record_2() -> User {
    User {
        id: 2,
        name: String::from("Jan"),
        lastname: String::from("Kowalski"),
    }
}

#[derive(Serialize)]
struct BadJson {
    bad: u32,
}

fn bad_json() -> BadJson {
    BadJson { bad: 223 }
}

#[tokio::test]
async fn test_endpoint_get_all_users_json_good() {
    let (server, db) = test_server();
    let response = server.get("/users").await;

    response.assert_json(&db.lock().await.clone());
}

#[tokio::test]
async fn test_endpoint_get_all_users_code_good() {
    let (server, _) = test_server();
    let response = server.get("/users").await;

    response.assert_status_ok()
}

#[tokio::test]
async fn test_endpoint_get_single_user_json_good() {
    let (server, _) = test_server();
    let response = server.get("/users/1").await;

    response.assert_json(&stripped_from_full(record_1()))
}

#[tokio::test]
async fn test_endpoint_get_single_user_code_good() {
    let (server, _) = test_server();
    let response = server.get("/users/1").await;

    response.assert_status_ok()
}

#[tokio::test]
async fn test_endpoint_get_single_user_code_bad() {
    let (server, _) = test_server();
    let response = server.get("/users/2").await;

    response.assert_status(StatusCode::NO_CONTENT)
}

// TODO zapytac sie o to czy tyle wystarczy (czy uzyc tez geta zeby sprawdzic baze)
#[tokio::test]
async fn test_endpoint_post_user_code_good() {
    let (server, _) = test_server();
    let response = server.post("/users").json(&json!(record_2())).await;

    response.assert_status(StatusCode::CREATED)
}

#[tokio::test]
async fn test_endpoint_post_user_code_bad() {
    let (server, _) = test_server();
    let response = server.post("/users").json(&json!(bad_json())).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY)
}

#[tokio::test]
async fn test_endpoint_patch_user_code_good() {
    let (server, _) = test_server();
    let response = server
        .patch("/users/1")
        .json(&json!(patch_name_from_full(record_2())))
        .await;
    response.assert_status(StatusCode::NO_CONTENT)
}
#[tokio::test]
async fn test_endpoint_patch_user_code_bad() {
    let (server, _) = test_server();
    let response = server.patch("/users/1").json(&json!(bad_json())).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY)
}

#[tokio::test]
async fn test_endpoint_put_user_code_good() {
    let (server, _) = test_server();
    let response = server
        .put("/users/1")
        .json(&json!(stripped_from_full(record_2())))
        .await;

    response.assert_status(StatusCode::NO_CONTENT)
}

#[tokio::test]
async fn test_endpoint_put_user_code_bad() {
    let (server, _) = test_server();
    let response = server.put("/users/1").json(&json!(bad_json())).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY)
}

#[tokio::test]
async fn test_endpoint_delete_user_code_good() {
    let (server, _) = test_server();
    let response = server.delete("/users/1").await;

    response.assert_status(StatusCode::NO_CONTENT)
}

#[tokio::test]
async fn test_endpoint_delete_user_code_bad() {
    let (server, _) = test_server();
    let response = server.delete("/users/2").await;

    response.assert_status(StatusCode::BAD_REQUEST)
}
