use std::sync::Arc;

use crate::{
    handler::{
        delete_user_handler, get_user_all_handler, get_user_single_handler, patch_user_handler,
        post_user_handler, put_user_handler,
    },
    model::{self, User},
    Db,
};
use axum::{routing::get, Router};
use tokio::sync::Mutex;

pub fn create_router() -> Router<Arc<Mutex<Vec<User>>>> {
    Router::new()
        .route("/users", get(get_user_all_handler).post(post_user_handler))
        .route(
            "/users/:id",
            get(get_user_single_handler)
                .patch(patch_user_handler)
                .put(put_user_handler)
                .delete(delete_user_handler),
        )
}

pub fn create_db() -> Arc<Mutex<Vec<User>>> {
    model::user_db()
}

pub fn join_router_db(router: Router<Arc<Mutex<Vec<User>>>>, db: Db) -> Router {
    router.with_state(db)
}
