use axum::{
    routing::{get, post},
    Router,
};

mod handler_delete_user;
mod handler_get_user;
mod handler_login;
mod handler_signup;
mod handler_update_user;

pub fn routes_user() -> Router {
    Router::new()
        .route("/signup", post(handler_signup::handler_signup))
        .route("/login", post(handler_login::handler_login))
        .route(
            "/user/:id",
            get(handler_get_user::handler_get_user)
                .put(handler_update_user::handler_update_user)
                .delete(handler_delete_user::handler_delete_user),
        )
}
