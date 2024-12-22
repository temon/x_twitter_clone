use axum::{
    extract::{FromRequest, rejection::JsonRejection, Request},
    response::{Response, IntoResponse},
    Json,
    async_trait
};
use axum::extract::State;
use axum::http::StatusCode;
use serde::Deserialize;

#[axum::debug_handler]
pub async fn create_post(post: CreatePostPayload) {
}

// create post payload model
#[derive(Debug)]
pub struct CreatePostPayload {
    pub text: String,
    pub parent_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct CreatePostPayloadValidator {
    pub text: Option<String>,
}

#[async_trait]
impl<S> FromRequest<S> for CreatePostPayload
where
    Json<CreatePostPayloadValidator>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(post) = Json::<CreatePostPayloadValidator>::from_request(req, state)
            .await
            .map_err(|error| {
                tracing::error!(
                    "Failed to parse request: {}",
                    error.body_text()
                );
                error.status().into_response()
            })?;

        tracing::debug!("Parsed request: {:?}", post);
        
        let Some(text) = post.text else {
            return Err((StatusCode::BAD_REQUEST, "text is required").into_response());
        };

        // check if it's empty string
        if text.trim().is_empty() {
            return Err((StatusCode::BAD_REQUEST, "text cannot be empty").into_response());
        }

        // check if it's too long (max 255 characters)
        if text.len() > 255 {
            return Err((StatusCode::BAD_REQUEST, "text is too long").into_response());
        }

        Ok(Self {
            text,
            parent_id: None,
        })
    }

}


