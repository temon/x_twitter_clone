use crate::db::queries::Post;
use crate::db::queries;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

pub async fn get_all_top_level(
    state: State<AppState>,
) -> Result<Json<Vec<Post>>, (StatusCode, &'static str)> {
    let posts = queries::get_all_top_level(state.db.clone())
        .await
        .map_err(|err| {
            tracing::error!("Failed to get all top level posts: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get all top level posts")
        })?;
    Ok(Json(posts))
}
