use axum::{extract::Path, http::StatusCode};
use tracing::debug;

use crate::{
    error::Result,
    responses::{AppResponse, StandardResponse},
};

pub async fn handler_delete_user(Path(user_id): Path<u32>) -> Result<AppResponse> {
    debug!("{:<12 } - handler_delete_user", "HANDLER");

    Ok(AppResponse::Standard(
        StatusCode::OK,
        StandardResponse::new(StatusCode::OK.into(), "request successful"),
    ))
}
