mod post;

use axum::Router;
use crate::router::post::create_post_router;

pub fn create_main_router() -> Router {
    Router::new()
        .nest("/api/v1/post", create_post_router())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
