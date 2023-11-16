use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub lastname: String,
}

pub type DB = Arc<Mutex<Vec<User>>>;

pub fn user_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct StripedUser {
    pub name: String,
    pub lastname: String,
}

#[derive(Deserialize)]
pub struct PatchUserSchema {
    pub name: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Serialize)]
pub struct NamePatch {
    pub name: String,
}
