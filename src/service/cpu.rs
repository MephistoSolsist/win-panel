use sysinfo::{CpuExt, SystemExt};
pub struct CpuInfo {
    pub name: String,
    pub cpu_usage: f32,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
}
pub fn get_cpu_info() -> Vec<CpuInfo> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_cpu();
    let mut result = vec![];
    for cpu in sys.cpus() {
        result.push(CpuInfo {
            name: cpu.name().to_string(),
            cpu_usage: cpu.cpu_usage(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        });
    }
    result
}
