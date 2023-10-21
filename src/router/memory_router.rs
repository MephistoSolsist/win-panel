use crate::service::get_mem_info;
use axum::{response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct MemoryInfo {
    memory_load: u32,
    total_phys: usize,
    avail_phys: usize,
    total_page_file: usize,
    avail_page_file: usize,
    total_virtual: usize,
    avail_virtual: usize,
}

pub fn memory_router_setup() -> Router {
    Router::new().route("/memory", get(memory))
}

async fn memory() -> Json<Value> {
    let info = get_mem_info();
    Json(json!(MemoryInfo {
        memory_load: info.dw_memory_load,
        total_phys: info.ull_total_phys,
        avail_phys: info.ull_avail_phys,
        total_page_file: info.ull_total_page_file,
        avail_page_file: info.ull_avail_page_file,
        total_virtual: info.ull_total_virtual,
        avail_virtual: info.ull_avail_virtual,
    }))
}
