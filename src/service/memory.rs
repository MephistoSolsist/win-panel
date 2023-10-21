use winapi::um::winbase;
pub struct Memorynfo {
    pub dw_memory_load: u32,
    pub ull_total_phys: usize,
    pub ull_avail_phys: usize,
    pub ull_total_page_file: usize,
    pub ull_avail_page_file: usize,
    pub ull_total_virtual: usize,
    pub ull_avail_virtual: usize,
}
pub fn get_mem_info() -> Memorynfo {
    unsafe {
        let mut status: winbase::MEMORYSTATUS = std::mem::zeroed();
        status.dwLength = std::mem::size_of::<winbase::MEMORYSTATUS>() as u32;
        winbase::GlobalMemoryStatus(&mut status);
        println!("Memory Load: {}", status.dwMemoryLoad);
        println!("Total Physical: {}", status.dwTotalPhys);
        println!("Available Physical: {}", status.dwAvailPhys);
        println!("Total Page File: {}", status.dwTotalPageFile);
        println!("Available Page File: {}", status.dwAvailPageFile);
        println!("Total Virtual: {}", status.dwTotalVirtual);
        println!("Available Virtual: {}", status.dwAvailVirtual);
        return Memorynfo {
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
