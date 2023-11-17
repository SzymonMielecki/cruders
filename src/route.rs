use crate::{
    handler::{
        delete_user_handler, get_user_all_handler, get_user_single_handler, patch_user_handler,
        post_user_handler, put_user_handler,
    },
    model::Db,
};
use axum::{routing::get, Router};

pub fn create_router() -> Router<Db> {
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

pub fn join_router_db(router: Router<Db>, db: Db) -> Router {
    router.with_state(db)
}
