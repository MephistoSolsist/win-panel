mod router;
mod service;
use router::setup_router;

#[tokio::main]
async fn main() {
    let app = setup_router();
    let url = "0.0.0.0:3000";
    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
