use serde_json::json;
use surrealdb::Result;

use crate::{
    model::{StripedUser, User},
    test_helper::{
        patch_name_from_full, record_1, record_1_patched, record_2, record_2_from_id,
        stripped_from_full, test_db_empty_raw, test_db_raw, test_server,
    },
};

#[tokio::test]
async fn test_get_all_no_modify_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.get("/users").await;

    response.assert_json(&json!(test_db_raw()));
    Ok(())
}

#[tokio::test]
async fn test_get_all_post_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let id_res = server
        .post("/users")
        .json(&json!(stripped_from_full(record_2())))
        .await;
    let response = server.get("/users").await;

    let json = response.json::<Vec<User>>();

    let id: String = id_res.text();

    assert!(json.contains(&record_2_from_id(id)));
    Ok(())
}

#[tokio::test]
async fn test_get_single_no_modify_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let response = server.get("/users/ebk6yszjd43bl4k2sry1").await;

    response.assert_json(&json!(stripped_from_full(record_1())));
    Ok(())
}

#[tokio::test]
async fn test_get_single_post_good() -> Result<()> {
    let (server, _) = test_server().await?;
    let id_res = server
        .post("/users")
        .json(&json!(stripped_from_full(record_2())))
        .await;

    let id: String = id_res.text();

    let path = "/users/".to_string() + &id;

    let response = server.get(&path).await;

    let json = response.json::<StripedUser>();

    assert_eq!(json, stripped_from_full(record_2_from_id(id)));
    Ok(())
}

#[tokio::test]
async fn test_get_single_patch_good() -> Result<()> {
    let (server, _) = test_server().await?;
    server
        .patch("/users/ebk6yszjd43bl4k2sry1")
        .json(&json!(patch_name_from_full(record_2())))
        .await;
    let response = server.get("/users/ebk6yszjd43bl4k2sry1").await;

    response.assert_json(&json!(stripped_from_full(record_1_patched())));
    Ok(())
}

#[tokio::test]
async fn test_get_single_put_modify_good() -> Result<()> {
    let (server, _) = test_server().await?;
    server
        .put("/users/ebk6yszjd43bl4k2sry1")
        .json(&json!(stripped_from_full(record_2())))
        .await;
    let response = server.get("/users/ebk6yszjd43bl4k2sry1").await;

    response.assert_json(&json!(stripped_from_full(record_2())));
    Ok(())
}

#[tokio::test]
async fn test_get_single_put_new_good() -> Result<()> {
    let (server, _) = test_server().await?;
    server
        .put("/users/mswwd6mcdx0zwxci5hlr")
        .json(&json!(stripped_from_full(record_2())))
        .await;
    let response = server.get("/users/mswwd6mcdx0zwxci5hlr").await;

    response.assert_json(&json!(stripped_from_full(record_2())));
    Ok(())
}

#[tokio::test]
async fn test_get_single_delete_good() -> Result<()> {
    let (server, _) = test_server().await?;
    server.delete("/users/ebk6yszjd43bl4k2sry1").await;
    let response = server.get("/users").await;

    response.assert_json(&json!(test_db_empty_raw()));
    Ok(())
}
