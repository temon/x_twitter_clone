use crate::db::connect::DB;
use eyre::Result;
use serde::{Deserialize, Serialize};

pub async fn insert_post(db: DB, text: &str, parent_id: Option<i32>) -> Result<i32> {
    let row = sqlx::query!(
        r#"
        INSERT INTO post (text, parent_id)
        VALUES ($1, $2)
        RETURNING id
        "#,
        text,
        parent_id
    )
    .fetch_one(&db)
    .await?;

    Ok(row.id)
}

pub async fn get_all_top_level(db: DB) -> Result<Vec<Post>> {
    let rows = sqlx::query_as!(
        Post,
        r#"
        SELECT
            id,
            text,
            likes,
            parent_id 
        FROM post
        WHERE parent_id IS NULL
        ORDER BY id DESC
        "#
    )
    .fetch_all(&db)
    .await?;

    Ok(rows)
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub text: String,
    pub likes: Option<i32>,
    pub parent_id: Option<i32>,
}