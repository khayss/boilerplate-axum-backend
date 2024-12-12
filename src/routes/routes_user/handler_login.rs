use axum::{http::StatusCode, Json};
use serde::Deserialize;
use tracing::debug;

use crate::{
    error::Result,
    responses::{AppResponse, StandardResponse},
};

pub async fn handler_login(Json(payload): Json<LoginPayload>) -> Result<AppResponse> {
    debug!("{:<12 } - handler_login", "HANDLER");

    let LoginPayload { email, password } = payload;

    Ok(AppResponse::Standard(
        StatusCode::OK,
        StandardResponse::new(StatusCode::OK.into(), "login_successfull"),
    ))
}

// LOGIN PAYLOAD
#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}
