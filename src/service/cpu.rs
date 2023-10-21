pub struct CpuInfo {}
pub fn get_cpu_info() -> CpuInfo {
    // println!("total memory: {} bytes", sys.total_memory());
    // println!("used memory : {} bytes", sys.used_memory());
    // println!("total swap  : {} bytes", sys.total_swap());
    // println!("used swap   : {} bytes", sys.used_swap());

    unsafe {
        return CpuInfo {};
    }
}
