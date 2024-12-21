pub mod create;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn create_post_router() -> Router {
    Router::new()
        .route("/", post(create::create_post))
}