use axum::Router;

mod routes_health;
mod routes_hello;
mod routes_user;

pub fn routes_all() -> Router {
    let routes_api = Router::new().nest("/users", routes_user::routes_user());

    Router::new()
        .merge(routes_health::routes_health())
        .merge(routes_hello::routes_hello())
        .nest("/api", routes_api)
}
