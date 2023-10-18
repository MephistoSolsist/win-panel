use std::fs;
use std::io;
use std::path::Path;
pub struct DriveInfo {
    drive_name: String,
}
impl DriveInfo {
    pub fn new(drive_name: String) -> Self {
        Self { drive_name }
    }
    /// 卷标 和 分区格式 such as NTFS or FAT32
    pub fn drive_info(&self) -> io::Result<(String, String)> {
        //winapi::um::fileapi::GetVolumeInformationW
        let drive_name = self.drive_name.clone() + ":\\";
        let drive_name = drive_name.encode_utf16().collect::<Vec<_>>();
        let mut volume_name_buffer = [0u16; 256];
        let mut file_system_name_buffer = [0u16; 256];
        unsafe {
            let result = winapi::um::fileapi::GetVolumeInformationW(
                drive_name.as_ptr(),
                volume_name_buffer.as_mut_ptr(),
                volume_name_buffer.len() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                file_system_name_buffer.as_mut_ptr(),
                file_system_name_buffer.len() as u32,
            );
            if result != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok((
                    String::from_utf16_lossy(&file_system_name_buffer),
                    String::from_utf16_lossy(&volume_name_buffer),
                ))
            }
        }
    }
}
/// 获取所有逻辑驱动器的盘符
pub fn get_drive_labels() -> Vec<char> {
    let mut result = vec![];
    unsafe {
        let mut drives = winapi::um::fileapi::GetLogicalDrives();
        //u32，总共32位，每一位代表一个驱动器，1表示可用，0表示不可用
        for drive_letter in b'A'..=b'Z' {
            if (drives & 1) != 0 {
                result.push(drive_letter as char);
            }
            drives >>= 1;
        }
    }
    result
}
