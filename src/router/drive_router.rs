use axum::{
    response::Json,
    routing::get,
    Router,
};
use crate::service::get_drives;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct Drive {
    name: String,
    size: String,
    free: String,
    used_percent: f64,
}

pub fn drive_router_setup() -> Router<> {
    Router::new()
        .route("/disk", get(disk))
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