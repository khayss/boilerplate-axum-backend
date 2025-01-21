use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;

pub async fn db(config: &Config) -> Pool<Postgres> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.POSTGRES_USER,
        config.POSTGRES_PASSWORD,
        config.POSTGRES_HOST,
        config.POSTGRES_PORT,
        config.POSTGRES_DB
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to datbase"))
}
