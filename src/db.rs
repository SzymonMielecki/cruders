use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::Db;

#[derive(Debug, Serialize)]
struct User<'a> {
    name: &'a str,
    lastname: &'a str,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: u32,
}

pub async fn init_users_db() -> surrealdb::Result<Db> {
    Surreal::new::<Mem>(()).await
}

pub async fn create_user(db: Db) -> surrealdb::Result<Vec<Record>> {
    db.use_ns("users").use_db("users").await?;
    let created: Vec<Record> = db
        .create("user")
        .content(User {
            name: "John",
            lastname: "Doe",
        })
        .await?;
    Ok(created)
}

pub async fn select_all_users(db: Db) -> surrealdb::Result<Vec<Record>> {
    db.use_ns("users").use_db("users").await?;
    let users: Vec<Record> = db.select("users").await?;

    Ok(users)
}

pub async fn select_single_user(db: Db, id: u32) -> surrealdb::Result<Record> {
    let users = select_all_users(db).await?;
    if let Some(user) = select_all_users(db)
        .await?
        .into_iter()
        .find(|rec| rec.id == id)
    {
        Ok(user)
    } else {
        Err(anyhow!("id: {id} is missing"))
    }
}
