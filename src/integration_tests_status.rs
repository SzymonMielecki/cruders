use axum::http::StatusCode;
use serde_json::json;
use surrealdb::Result;

use super::test_helper::{
    patch_name_from_full, record_2, stripped_from_full, test_server, BadJson,
};

#[tokio::test]
async fn test_endpoint_get_all_users_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.get("/users").await;

    response.assert_status(StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_get_single_user_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.get("/users/ebk6yszjd43bl4k2sry1").await;

    response.assert_status(StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_get_single_user_bad() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.get("/users/mswwd6mcdx0zwxci5hlr").await;

    response.assert_status(StatusCode::BAD_REQUEST);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_post_user_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.post("/users").json(&json!(record_2())).await;

    response.assert_status(StatusCode::CREATED);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_post_user_bad() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.post("/users").json(&json!(BadJson::default())).await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_patch_user_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server
        .patch("/users/ebk6yszjd43bl4k2sry1")
        .json(&json!(patch_name_from_full(record_2())))
        .await;

    response.assert_status(StatusCode::NO_CONTENT);
    Ok(())
}
#[tokio::test]
async fn test_endpoint_patch_user_bad() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server
        .patch("/users/ebk6yszjd43bl4k2sry1")
        .json(&json!(BadJson::default()))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_put_user_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server
        .put("/users/ebk6yszjd43bl4k2sry1")
        .json(&json!(stripped_from_full(record_2())))
        .await;

    response.assert_status(StatusCode::NO_CONTENT);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_put_user_bad() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server
        .put("/users/ebk6yszjd43bl4k2sry1")
        .json(&json!(BadJson::default()))
        .await;

    response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_delete_user_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.delete("/users/ebk6yszjd43bl4k2sry1").await;

    response.assert_status(StatusCode::NO_CONTENT);
    Ok(())
}

#[tokio::test]
async fn test_endpoint_delete_user_bad() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.delete("/users/mswwd6mcdx0zwxci5hlr").await;

    response.assert_status(StatusCode::BAD_REQUEST);
    Ok(())
}
