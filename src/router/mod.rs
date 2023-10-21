mod info;
use axum::Router;
use tower_http::cors::CorsLayer;

pub fn setup_router() -> Router {
    Router::new()
        .merge(info::memory_router_setup())
        .layer(CorsLayer::permissive())
}
