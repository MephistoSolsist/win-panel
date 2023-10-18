extern crate winapi;
mod service;
use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use service::get_drives;


#[derive(Serialize, Deserialize)]
struct Drive {
    name: String,
    size: String,
    free: String,
    used_percent: f64,
}


async fn disk() -> Json<Value> {
    get_drives();
    let c = Drive {
        name: String::from("C:"),
        size: "100.00GB".to_string(),
        free: "50.00".to_string(),
        used_percent: 50.0,
    };
    let d = Drive {
        name: String::from("D:"),
        size: "120.00GB".to_string(),
        free: "63.00GB".to_string(),
        used_percent: 47.5,
    };
    let drive_list = [&c,&d];
    Json(json!(drive_list))
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/disk", get(disk));
    let url = &"0.0.0.0:3000".parse().unwrap();
    axum::Server::bind(url)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
