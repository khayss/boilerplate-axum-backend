use axum::{extract::Query, http::StatusCode, routing::get, Router};
use tracing::debug;

use crate::{error::Result, responses::AppResponse};

pub fn routes_hello() -> Router {
    Router::new().route("/hello", get(handler_hello))
}

async fn handler_hello(Query(name): Query<Option<String>>) -> Result<AppResponse> {
    debug!("{:<12} >> handler_health", "HANDLER");

    let suffix = name.as_deref().unwrap_or("World");
    let mes = format!("Hello, {}!", suffix);

    Ok(AppResponse::Simple(StatusCode::OK, String::from(mes)))
}
