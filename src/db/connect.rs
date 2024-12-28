use eyre::Result;
use sqlx::{Connection, Database, Pool, postgres::PgPoolOptions, Postgres};

pub type DB = Pool<Postgres>;

// create function to connect to database with sqlx
// we can use this in the application
pub async fn connect_to_db(database_uri: &str) -> Result<DB> {
    Ok(
        PgPoolOptions::new()
        .max_connections(5)
        .connect(database_uri)
        .await?
    )
}