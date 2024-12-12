use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod routes;
mod responses;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .without_time()
        .init();

    let app = routes::routes_all();

    let port = 4000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("{:<12} >> app Listening on: {addr}", "LISTENING");

    axum::serve(listener, app).await.unwrap();
}
