use axum::{
    extract::{FromRequest, rejection::JsonRejection, Request},
    response::{Response, IntoResponse},
    Json,
    async_trait
};
use axum::extract::State;
use serde::Deserialize;

#[axum::debug_handler]
pub async fn create_post(post: CreatePostPayload) {
    todo!();
}

// create post payload model
#[derive(Deserialize, Debug)]
pub struct CreatePostPayload {
    pub text: String,
    pub parent_id: Option<i32>,
}

#[async_trait]
impl<S> FromRequest<S> for CreatePostPayload
where
    Json<Self>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(post) = Json::<CreatePostPayload>::from_request(req, state)
            .await
            .map_err(|error| {
                tracing::error!(
                    "Failed to parse request: {}",
                    error.body_text()
                );
                error.status().into_response()
            })?;

        tracing::debug!("Parsed request: {:?}", post);

        todo!();
    }

}


