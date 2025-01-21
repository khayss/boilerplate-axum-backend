use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;

pub async fn db(config: &Config) -> Pool<Postgres> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.DB_USERNAME, config.DB_PASSWORD, config.DB_HOST, config.DB_PORT, config.DB_NAME
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to datbase"))
}
