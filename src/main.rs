use std::{net::SocketAddr, sync::Arc};

use config::Config;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod controllers;
mod database;
mod error;
mod models;
mod responses;
mod routes;
mod types;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .without_time()
        .init();

    let config = Config::init();
    let app_state = Arc::new(AppState::new(&config).await);
    let app = routes::routes(app_state);

    let port = 4000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("{:<12} >> app Listening on: {addr}", "LISTENING");

    axum::serve(listener, app).await.unwrap();
}

struct AppState {
    db_pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new(config: &Config) -> Self {
        let db_pool = database::db(config).await;

        Self { db_pool }
    }
}
