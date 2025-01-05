pub mod create;
mod get_all_top_level;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use crate::state::AppState;

pub fn create_post_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create::create_post))
        .route("/", get(get_all_top_level::get_all_top_level))
        .with_state(state)
}