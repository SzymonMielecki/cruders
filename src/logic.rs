use crate::{
    db::{delete_user, get_all_users, get_single_user, patch_user, post_user, put_user},
    model::{Db, OutUser, PatchUserSchema, StripedUser, User},
};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

pub trait StateTrait {
    fn get_single_user_logic(
        &self,
        id: String,
    ) -> impl std::future::Future<Output = surrealdb::Result<OutUser>> + Send;
    fn get_all_users_logic(
        &self,
    ) -> impl std::future::Future<Output = surrealdb::Result<Vec<OutUser>>> + Send;
    fn post_user_logic(
        &self,
        body: StripedUser,
    ) -> impl std::future::Future<Output = surrealdb::Result<String>> + Send;
    fn patch_user_logic(
        &self,
        id: String,
        body: PatchUserSchema,
    ) -> impl std::future::Future<Output = surrealdb::Result<OutUser>> + Send;
    fn put_user_logic(
        &self,
        id: String,
        body: StripedUser,
    ) -> impl std::future::Future<Output = surrealdb::Result<OutUser>> + Send;
    fn delete_user_logic(
        &self,
        id: String,
    ) -> impl std::future::Future<Output = surrealdb::Result<OutUser>> + Send;
}

pub enum Error {
    BadRequest,
    NotFound,
}
impl AppState {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
impl StateTrait for AppState {
    async fn get_single_user_logic(&self, id: String) -> surrealdb::Result<OutUser> {
        get_single_user(&self.db, id).await
    }

    async fn get_all_users_logic(&self) -> surrealdb::Result<Vec<OutUser>> {
        get_all_users(&self.db).await
    }

    async fn post_user_logic(&self, body: StripedUser) -> surrealdb::Result<String> {
        if body.group != "user" && body.group != "admin" && body.group != "premium" {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
                "Bad Group".into(),
            )));
        }
        if !(1900..=2024).contains(&body.birthyear) {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
                "Bad Birthyear".into(),
            )));
        }

        let record = User {
            id: None,
            name: body.name,
            lastname: body.lastname,
            birthyear: body.birthyear,
            group: body.group,
        };

        post_user(&self.db, record).await
    }

    async fn patch_user_logic(
        &self,
        id: String,
        body: PatchUserSchema,
    ) -> surrealdb::Result<OutUser> {
        if body.group.is_some()
            && body.group != Some("user".into())
            && body.group != Some("admin".into())
            && body.group != Some("premium".into())
        {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
                "Bad Group".into(),
            )));
        }
        if body.birthyear.is_some() && !(1900..=2024).contains(&body.birthyear.expect("birthyear"))
        {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
                "Bad Birthyear".into(),
            )));
        }
        patch_user(&self.db, id, body).await
    }
    async fn put_user_logic(&self, id: String, body: StripedUser) -> surrealdb::Result<OutUser> {
        if body.group != "user" && body.group != "admin" && body.group != "premium" {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
                "Bad Group".into(),
            )));
        }
        if !(1900..=2024).contains(&body.birthyear) {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
                "Bad Birthyear".into(),
            )));
        }
        put_user(&self.db, id, body).await
    }
    async fn delete_user_logic(&self, id: String) -> surrealdb::Result<OutUser> {
        delete_user(&self.db, id).await
    }
}
