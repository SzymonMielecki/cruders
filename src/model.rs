use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db as LocalDb, sql::Thing, Surreal};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub lastname: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct StripedUser {
    pub name: String,
    pub lastname: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PatchUserSchema {
    pub name: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Serialize)]
pub struct PatchUserName {
    pub name: String,
}
#[derive(Serialize)]
pub struct PatchUserLastname {
    pub lastname: String,
}

#[derive(Serialize)]
pub struct NamePatch {
    pub name: String,
}

pub type Db = Surreal<LocalDb>;
