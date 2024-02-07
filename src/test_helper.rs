use axum_test::TestServer;
use serde::Serialize;
use surrealdb::{
    sql::{Id, Thing},
    Result,
};

use crate::{
    db::init_users_db,
    model::{Db, OutUser, PatchUserSchema, StripedUser, User},
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

pub fn test_db_raw() -> Vec<OutUser> {
    vec![OutUser::from(record_1())]
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
        birthyear: 1984,
        group: String::from("admin"),
    }
}
pub fn record_1_id() -> String {
    "ebk6yszjd43bl4k2sry1".into()
}

pub fn record_1_patched() -> User {
    User {
        id: Some(Thing {
            tb: "users".into(),
            id: Id::String("ebk6yszjd43bl4k2sry1".into()),
        }),
        birthyear: 1984,
        group: "admin".into(),
        name: String::from("Jan"),
        lastname: String::from("Doe"),
    }
}

pub fn stripped_from_full(user: User) -> StripedUser {
    StripedUser {
        name: user.name,
        lastname: user.lastname,
        birthyear: user.birthyear,
        group: user.group.to_string(),
    }
}

pub fn patch_name_from_full(user: User) -> PatchUserSchema {
    PatchUserSchema {
        name: Some(user.name),
        lastname: None,
        birthyear: None,
        group: None,
    }
}

pub fn record_2() -> User {
    User {
        id: None,
        name: String::from("Jan"),
        lastname: String::from("Kowalski"),
        birthyear: 2003,
        group: "user".into(),
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
        birthyear: 2003,
        group: "user".into(),
    }
}

#[derive(Serialize)]
pub struct BadJson {
    bad: u32,
}

impl Default for BadJson {
    fn default() -> Self {
        Self { bad: 223 }
    }
}
