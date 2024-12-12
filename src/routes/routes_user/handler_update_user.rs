use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;
use tracing::debug;

use crate::{
    error::Result,
    responses::{AppResponse, StandardResponse},
};

pub async fn handler_update_user(
    Path(user_id): Path<u32>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<AppResponse> {
    debug!("{:<12 } - handler_update_user", "HANDLER");

    match payload {
        UpdateUserPayload::UpdateNames {
            firstname,
            lastname,
        } => {
            debug!("Updating names: {} {}", firstname, lastname);
        }
        UpdateUserPayload::UpdateUsername { username } => {
            debug!("Updating username: {}", username);
        }
        UpdateUserPayload::UpdateEmail { email } => {
            debug!("Updating email: {}", email);
        }
        UpdateUserPayload::UpdatePassword { password } => {
            debug!("Updating password: {}", password);
        }
    }

    Ok(AppResponse::Standard(
        StatusCode::OK,
        StandardResponse::new(StatusCode::OK.into(), "request successful"),
    ))
}

#[derive(Debug, Deserialize)]
pub enum UpdateUserPayload {
    UpdateNames { firstname: String, lastname: String },
    UpdateUsername { username: String },
    UpdateEmail { email: String },
    UpdatePassword { password: String },
}
