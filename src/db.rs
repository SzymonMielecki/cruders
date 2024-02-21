use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

use crate::model::{
    Db, OutUser, PatchUserBirthyear, PatchUserGroup, PatchUserLastname, PatchUserName,
    PatchUserSchema, StripedUser, User,
};

pub async fn init_users_db() -> surrealdb::Result<Db> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("users").use_db("users").await?;
    Ok(db)
}

pub async fn post_user(db: &Db, user: User) -> surrealdb::Result<String> {
    db.use_ns("users").use_db("users").await?;
    let tmp: Vec<User> = db.create("users").content(user).await?;

    Ok(tmp[0].id.as_ref().unwrap().id.to_string())
}

pub async fn get_all_users(db: &Db) -> surrealdb::Result<Vec<OutUser>> {
    db.use_ns("users").use_db("users").await?;
    let users: Vec<User> = db.select("users").await?;
    let out_users: Vec<OutUser> = users.into_iter().map(|u| u.into()).collect();

    Ok(out_users)
}

pub async fn get_single_user(db: &Db, id: String) -> surrealdb::Result<OutUser> {
    db.use_ns("users").use_db("users").await?;
    let opt: Option<User> = db.select(("users", &id)).await?;
    println!("{:?}", opt);
    match opt {
        Some(user) => Ok(user.into()),
        None => Err(surrealdb::Error::Db(surrealdb::error::Db::PaNotFound {
            value: id,
        })),
    }
}

pub async fn patch_user(db: &Db, id: String, body: PatchUserSchema) -> surrealdb::Result<OutUser> {
    db.use_ns("users").use_db("users").await?;
    println!("{:?}", body);
    if body.lastname.is_none()
        && body.name.is_none()
        && body.birthyear.is_none()
        && body.group.is_none()
    {
        return Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
            "Bad Request".into(),
        )));
    }

    let mut opt: Option<User> = None;
    if body.name.is_some() {
        opt = db
            .update(("users", id.clone()))
            .merge(PatchUserName {
                name: body.name.unwrap(),
            })
            .await?;
    }
    if body.lastname.is_some() {
        opt = db
            .update(("users", id.clone()))
            .merge(PatchUserLastname {
                lastname: body.lastname.unwrap(),
            })
            .await?;
    }
    if body.birthyear.is_some() {
        opt = db
            .update(("users", id.clone()))
            .merge(PatchUserBirthyear {
                birthyear: body.birthyear.unwrap(),
            })
            .await?;
    }
    if body.group.is_some() {
        opt = db
            .update(("users", id.clone()))
            .merge(PatchUserGroup {
                group: body.group.unwrap(),
            })
            .await?;
    }

    match opt {
        Some(user) => Ok(user.into()),
        None => Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
            "Bad request".into(),
        ))),
    }
}

pub async fn put_user(db: &Db, id: String, body: StripedUser) -> surrealdb::Result<OutUser> {
    db.use_ns("users").use_db("users").await?;

    let opt: Option<User> = db.select(("users", &id)).await?;

    match opt {
        Some(_) => {
            let tmp: Option<User> = db.update(("users", id)).merge(body).await?;
            Ok(tmp.unwrap().into())
        }
        None => {
            let tmp: Option<User> = db.create(("users", id)).content(body).await?;
            Ok(tmp.unwrap().into())
        }
    }
}

pub async fn delete_user(db: &Db, id: String) -> surrealdb::Result<OutUser> {
    db.use_ns("users").use_db("users").await?;

    let opt: Option<User> = db.select(("users", &id)).await?;

    match opt {
        Some(_) => {
            let tmp: Option<User> = db.delete(("users", &id)).await?;
            Ok(tmp.unwrap().into())
        }
        None => Err(surrealdb::Error::Api(surrealdb::error::Api::Http(
            "Bad request".into(),
        ))),
    }
}
