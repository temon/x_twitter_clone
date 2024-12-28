use crate::db::connect::DB;
use eyre::Result;

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