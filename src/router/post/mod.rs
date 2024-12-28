pub mod create;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use crate::state::AppState;

pub fn create_post_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create::create_post))
        .with_state(state)
}