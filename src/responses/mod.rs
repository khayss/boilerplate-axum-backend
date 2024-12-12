use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppResponse {
    Standard(StatusCode, StandardResponse),
    GetUser(StatusCode, GetUserResponse),
    Simple(StatusCode, String),
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    pub message: String,
    pub status: u16,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct StandardResponse {
    message: String,
    status: u16,
}

impl StandardResponse {
    pub fn new(status: u16, message: &str) -> Self {
        Self {
            message: message.to_string(),
            status,
        }
    }
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> Response {
        match self {
            AppResponse::Standard(status_code, data) => (status_code, Json(data)).into_response(),
            AppResponse::GetUser(status_code, data) => (status_code, Json(data)).into_response(),
            AppResponse::Simple(status_code, message) => (status_code, message).into_response(),
        }
    }
}
