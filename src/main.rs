mod router;
mod service;
use router::setup_router;
use sysinfo::{ComponentExt, SystemExt};

#[tokio::main]
async fn main() {
    let sys = sysinfo::System::new_all();
    for c in sys.boot_time() {
        println!("{:?}", c);
    }
    let app = setup_router();
    let url = "0.0.0.0:12345";
    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
