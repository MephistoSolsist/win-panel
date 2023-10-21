use sysinfo::{NetworkExt, SystemExt};
pub struct NetworkInfo {
    pub name: String,
    pub total_received: u64,
    pub total_transmitted: u64,
}
pub fn get_network_info() -> Vec<NetworkInfo> {
    let sys = sysinfo::System::new_all();
    // sys.refresh_networks();
    let networks = sys.networks();
    let mut result = vec![];
    for (name, info) in networks {
        result.push(NetworkInfo {
            name: name.to_string(),
            total_received: info.total_received(),
            total_transmitted: info.total_transmitted(),
        });
    }
    result
}
