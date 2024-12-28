mod post;

use axum::Router;
use crate::router::post::create_post_router;
use crate::state::AppState;

pub fn create_main_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/post", create_post_router(state))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
