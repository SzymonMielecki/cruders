use crate::{
    handler::{
        delete_user_handler, get_user_all_handler, get_user_single_handler, patch_user_handler,
        post_user_handler, put_user_handler,
    },
    logic::AppState,
};
use axum::routing::{get, Router};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_user_all_handler).post(post_user_handler))
        .route(
            "/users/:id",
            get(get_user_single_handler)
                .patch(patch_user_handler)
                .put(put_user_handler)
                .delete(delete_user_handler),
        )
        .with_state(state)
}
