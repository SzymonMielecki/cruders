use axum_test::TestServer;
use serde::Serialize;
use surrealdb::{
    sql::{Id, Thing},
    Result,
};

use crate::{
    db::init_users_db,
    model::{Db, NamePatch, StripedUser, User},
    route::{create_router, join_router_db},
};

pub async fn test_server() -> Result<(TestServer, Db)> {
    let db = init_users_db().await?;
    db.use_ns("users").use_db("users").await?;

    let _: Option<User> = db
        .create(("users", "ebk6yszjd43bl4k2sry1"))
        .content(stripped_from_full(record_1()))
        .await?;

    let app = join_router_db(create_router(), db.clone());

    let server = TestServer::new(app).unwrap();

    Ok((server, db))
}

pub fn test_db_raw() -> Vec<User> {
    vec![record_1()]
}

pub fn test_db_empty_raw() -> Vec<User> {
    vec![]
}

pub fn record_1() -> User {
    User {
        id: Some(Thing {
            tb: "users".into(),
            id: Id::String("ebk6yszjd43bl4k2sry1".into()),
        }),
        name: String::from("John"),
        lastname: String::from("Doe"),
    }
}

pub fn record_1_patched() -> User {
    User {
        id: Some(Thing {
            tb: "users".into(),
            id: Id::String("ebk6yszjd43bl4k2sry1".into()),
        }),
        name: String::from("Jan"),
        lastname: String::from("Doe"),
    }
}

pub fn stripped_from_full(user: User) -> StripedUser {
    StripedUser {
        name: user.name,
        lastname: user.lastname,
    }
}

pub fn patch_name_from_full(user: User) -> NamePatch {
    NamePatch { name: user.name }
}

pub fn record_2() -> User {
    User {
        id: None,

        name: String::from("Jan"),
        lastname: String::from("Kowalski"),
    }
}

pub fn record_2_from_id(id: String) -> User {
    User {
        id: Some(Thing {
            tb: "users".into(),
            id: Id::String(id),
        }),

        name: String::from("Jan"),
        lastname: String::from("Kowalski"),
    }
}

#[derive(Serialize)]
pub struct BadJson {
    bad: u32,
}
impl BadJson {
    pub fn new() -> BadJson {
        BadJson { bad: 223 }
    }
}
