mod drive_router;
mod memory_router;

use axum::Router;
use tower_http::cors::CorsLayer;

pub fn setup_router() -> Router {
    Router::new()
        .merge(drive_router::drive_router_setup())
        .layer(CorsLayer::permissive())
        .merge(memory_router::memory_router_setup())
        .layer(CorsLayer::permissive())
}
