use winapi::um::winbase;
pub struct MemoryInfo {
    pub dw_memory_load: u32,
    pub ull_total_phys: usize,
    pub ull_avail_phys: usize,
    pub ull_total_page_file: usize,
    pub ull_avail_page_file: usize,
    pub ull_total_virtual: usize,
    pub ull_avail_virtual: usize,
}
pub fn get_mem_info() -> MemoryInfo {
    unsafe {
        let mut status: winbase::MEMORYSTATUS = std::mem::zeroed();
        status.dwLength = std::mem::size_of::<winbase::MEMORYSTATUS>() as u32;
        winbase::GlobalMemoryStatus(&mut status);
        return MemoryInfo {
            dw_memory_load: status.dwMemoryLoad,
            ull_total_phys: status.dwTotalPhys as usize,
            ull_avail_phys: status.dwAvailPhys as usize,
            ull_total_page_file: status.dwTotalPageFile as usize,
            ull_avail_page_file: status.dwAvailPageFile as usize,
            ull_total_virtual: status.dwTotalVirtual as usize,
            ull_avail_virtual: status.dwAvailVirtual as usize,
        };
    }
}
