use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::Value;

#[derive(Debug)]
pub struct AppResponse {
    body: Value,
    status_code: StatusCode,
}

impl AppResponse {
    pub fn new(status_code: StatusCode, body: Value) -> Self {
        Self { status_code, body }
    }
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.body)).into_response()
    }
}
