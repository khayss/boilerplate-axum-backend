use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod health_route;
mod user_routes;

pub fn routes(app_state: Arc<AppState>) -> Router {
    let api_routes = Router::new().nest("/users", user_routes::routes(app_state));

    Router::new()
        .merge(health_route::route())
        .nest("/api", api_routes)
}
