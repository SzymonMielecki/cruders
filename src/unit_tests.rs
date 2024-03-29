use super::{
    db::{delete_user, get_all_users, get_single_user, patch_user, post_user, put_user},
    model::Db,
    test_helper::{patch_name_from_full, record_1_id, record_2, stripped_from_full, test_server},
};
use surrealdb::Result;
use tokio::test;

pub async fn test_db() -> Result<Db> {
    let db = test_server().await?.1.db;
    Ok(db)
}
#[test]
pub async fn test_func_get_all() -> Result<()> {
    let db = test_db().await?;
    assert!(get_all_users(&db).await.is_ok());
    Ok(())
}

#[test]
pub async fn test_func_post() -> Result<()> {
    let db = test_db().await?;
    assert!(post_user(&db, record_2()).await.is_ok());
    Ok(())
}

#[test]
pub async fn test_func_get_single() -> Result<()> {
    let db = test_db().await?;
    assert!(get_single_user(&db, record_1_id()).await.is_ok());
    Ok(())
}

#[test]
pub async fn test_func_patch() -> Result<()> {
    let db = test_db().await?;
    assert!(
        patch_user(&db, record_1_id(), patch_name_from_full(record_2()))
            .await
            .is_ok()
    );
    Ok(())
}

#[test]
pub async fn test_func_put() -> Result<()> {
    let db = test_db().await?;
    assert!(put_user(&db, record_1_id(), stripped_from_full(record_2()))
        .await
        .is_ok());
    Ok(())
}

#[test]
pub async fn test_func_delte() -> Result<()> {
    let db = test_db().await?;
    assert!(delete_user(&db, record_1_id()).await.is_ok());
    Ok(())
}
