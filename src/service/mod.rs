mod cpu;
mod drive;
mod memory;
mod networks;

pub use cpu::get_cpu_info;
pub use drive::get_drives_info;
pub use memory::get_mem_info;
pub use networks::get_network_info;
