use axum::{http::StatusCode, routing::get, Router};
use tracing::debug;

use crate::{error::Result, responses::AppResponse};

pub fn routes_health() -> Router {
    Router::new().route("/health", get(handler_health))
}

async fn handler_health() -> Result<AppResponse> {
    debug!("{:<12} >> handler_health", "HANDLER");

    Ok(AppResponse::Simple(StatusCode::OK, String::from("Healthy")))
}
