use axum::{http::StatusCode, Json};
use serde::Deserialize;
use tracing::debug;

use crate::{
    error::Result,
    responses::{AppResponse, StandardResponse},
};

pub async fn handler_signup(Json(payload): Json<SignupPayload>) -> Result<AppResponse> {
    debug!("{:<12 } - handler_signup", "HANDLER");

    let SignupPayload {
        email,
        first_name,
        last_name,
        password,
        username,
    } = payload;

    Ok(AppResponse::Standard(
        StatusCode::CREATED,
        StandardResponse::new(StatusCode::CREATED.into(), "User created successfully"),
    ))
}

// SIGNUP PAYLOAD
#[derive(Debug, Deserialize)]
pub struct SignupPayload {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    password: String,
}
