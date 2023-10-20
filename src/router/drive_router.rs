use crate::service::get_drives;
use axum::{response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct Drive {
    label_name: String,
    format: String,
    available_free_space: u64,
    total_size: u64,
    total_free_space: u64,
}

pub fn drive_router_setup() -> Router {
    Router::new().route("/disk", get(disk))
}

async fn disk() -> Json<Value> {
    let mut drive_list = vec![];
    for drive in get_drives() {
        match drive.get_all_info() {
            Ok(info) => {
                let drive = Drive {
                    label_name: info.label_name,
                    format: info.format,
                    available_free_space: info.available_free_space,
                    total_size: info.total_size,
                    total_free_space: info.total_free_space,
                };
                drive_list.push(drive);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Json(json!(drive_list))
}
