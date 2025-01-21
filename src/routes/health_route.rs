use axum::{http::StatusCode, routing::get, Router};
use serde_json::json;
use tracing::debug;

use crate::{error::Result, responses::AppResponse};

pub fn route() -> Router {
    Router::new().route("/health", get(handler))
}

async fn handler() -> Result<AppResponse> {
    debug!("{:<12} >> handler_health", "HANDLER");

    Ok(AppResponse::new(StatusCode::OK, json!("Healthy")))
}
