use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug, Serialize)]
pub enum AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}
