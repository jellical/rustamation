use axum::routing::get;
use axum::Router;

pub fn get_base_route() -> Router {
    Router::new().route("/health-check", get(|| async { "OK" }))
}
