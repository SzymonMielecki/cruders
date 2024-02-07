use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db as LocalDb, sql::Thing, Surreal};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub lastname: String,
    pub birthyear: u32,
    pub group: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OutUser {
    pub name: String,
    pub lastname: String,
    pub age: u32,
    pub group: String,
}

impl From<User> for OutUser {
    fn from(user: User) -> Self {
        Self {
            name: user.name,
            lastname: user.lastname,
            age: 2024 - user.birthyear,
            group: user.group,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct StripedUser {
    pub name: String,
    pub lastname: String,
    pub birthyear: u32,
    pub group: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PatchUserSchema {
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub birthyear: Option<u32>,
    pub group: Option<String>,
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
pub struct PatchUserBirthyear {
    pub birthyear: u32,
}
#[derive(Serialize)]
pub struct PatchUserGroup {
    pub group: String,
}

#[derive(Serialize)]
pub struct NamePatch {
    pub name: String,
}

pub type Db = Surreal<LocalDb>;
