use crate::service::get_cpu_info;
use crate::service::get_drives_info;
use crate::service::get_mem_info;
use crate::service::get_network_info;
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
#[derive(Serialize, Deserialize)]
struct Drive {
    label_name: String,
    format: String,
    available_free_space: u64,
    total_size: u64,
    total_free_space: u64,
}
async fn disk() -> Json<Value> {
    let mut drive_list = vec![];
    for drive in get_drives_info() {
        if !drive.get_is_fixed() {
            continue;
        }
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
#[derive(Serialize, Deserialize)]
struct CpuInfo {
    name: String,
    cpu_usage: f32,
    vendor_id: String,
    brand: String,
    frequency: u64,
}
async fn cpu() -> Json<Value> {
    let info = get_cpu_info();
    Json(json!(CpuInfo {
        name: info.name,
        cpu_usage: info.cpu_usage,
        vendor_id: info.vendor_id,
        brand: info.brand,
        frequency: info.frequency,
    }))
}
#[derive(Serialize, Deserialize)]
struct NetworkInfo {
    name: String,
    total_received: u64,
    total_transmitted: u64,
}
async fn network() -> Json<Value> {
    let networks = get_network_info();
    let mut result = vec![];
    for network in networks {
        result.push(NetworkInfo {
            name: network.name,
            total_received: network.total_received,
            total_transmitted: network.total_transmitted,
        });
    }
    Json(json!(result))
}

pub fn memory_router_setup() -> Router {
    Router::new()
        .route("/memory", get(memory))
        .route("/disk", get(disk))
        .route("/cpu", get(cpu))
        .route("/network", get(network))
}
