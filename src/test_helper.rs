use axum_test::TestServer;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    model::{NamePatch, StripedUser, User},
    route::{create_router, join_router_db},
};

pub fn test_server() -> (TestServer, Arc<Mutex<Vec<User>>>) {
    let db = test_db();
    let app = join_router_db(create_router(), &db);

    let server = TestServer::new(app).unwrap();

    (server, db)
}

pub fn test_db() -> Arc<Mutex<Vec<User>>> {
    Arc::new(Mutex::new(vec![record_1()]))
}

pub fn test_db_raw() -> Vec<User> {
    vec![record_1()]
}

pub fn test_db_empty_raw() -> Vec<User> {
    vec![]
}

pub fn test_db_pushed_raw() -> Vec<User> {
    vec![record_1(), record_2()]
}

pub fn record_1() -> User {
    User {
        id: 1,
        name: String::from("John"),
        lastname: String::from("Doe"),
    }
}

pub fn record_1_patched() -> User {
    User {
        id: 1,
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
        id: 2,
        name: String::from("Jan"),
        lastname: String::from("Kowalski"),
    }
}

#[derive(Serialize)]
pub struct BadJson {
    bad: u32,
}

pub fn bad_json() -> BadJson {
    BadJson { bad: 223 }
}
