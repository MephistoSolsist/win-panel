use axum::Router;
use super::drive_router;

pub fn setup_router() -> Router<> {
    Router::new().merge(drive_router::drive_router_setup())
}
