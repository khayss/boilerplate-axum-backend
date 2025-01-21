use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{controllers::user_controller::UserController, AppState};

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", post(UserController::create_user))
        .route(
            "/:id",
            post(UserController::get_user_by_id)
                .put(UserController::update_user)
                .delete(UserController::delete_user),
        )
        .route("/login", post(UserController::login_user))
        .with_state(app_state)
}
