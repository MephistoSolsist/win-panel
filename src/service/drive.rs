extern crate winapi;
use std::io;
use winapi::um::winnt::ULARGE_INTEGER;
pub struct DriveInfoData {
    pub label_name: String,
    pub format: String,
    pub available_free_space: u64,
    pub total_size: u64,
    pub total_free_space: u64,
}
pub struct DriveInfo {
    pub drive_name: String,
    encoded_root_path: Vec<u16>,
}
impl DriveInfo {
    pub fn new(drive_name: String) -> Self {
        Self {
            encoded_root_path: {
                let drive_name = drive_name.clone() + ":\\";
                drive_name.encode_utf16().collect::<Vec<_>>()
            },
            drive_name,
        }
    }
    pub fn get_is_fixed(&self) -> bool {
        unsafe {
            let result = winapi::um::fileapi::GetDriveTypeW(self.encoded_root_path.as_ptr());
            //https://learn.microsoft.com/zh-tw/windows/win32/api/fileapi/nf-fileapi-getdrivetypew
            // DRIVE_FIXED  3
            // 磁片磁碟機具有固定媒體;例如，硬碟或快閃磁片磁碟機。
            // 本地磁盘
            if result == 3 {
                return true;
            } else {
                return false;
            }
        }
    }
    pub fn get_all_info(&self) -> io::Result<DriveInfoData> {
        let info = self.drive_info()?;
        let usage = self.usage()?;
        Ok(DriveInfoData {
            label_name: info.1,
            format: info.0,
            available_free_space: usage.0,
            total_size: usage.1,
            total_free_space: usage.2,
        })
    }
    /// 卷标 和 分区格式 such as NTFS or FAT32
    pub fn drive_info(&self) -> io::Result<(String, String)> {
        //winapi::um::fileapi::GetVolumeInformationW
        let mut volume_name_buffer = [0u16; 256];
        let mut file_system_name_buffer = [0u16; 256];
        unsafe {
            let result = winapi::um::fileapi::GetVolumeInformationW(
                self.encoded_root_path.as_ptr(),
                volume_name_buffer.as_mut_ptr(),
                volume_name_buffer.len() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                file_system_name_buffer.as_mut_ptr(),
                file_system_name_buffer.len() as u32,
            );
            println!("result: {}", result);
            if 0 == result {
                Err(io::Error::last_os_error())
            } else {
                Ok((
                    String::from_utf16_lossy(&file_system_name_buffer),
                    String::from_utf16_lossy(&volume_name_buffer),
                ))
            }
        }
    }
    pub fn usage(&self) -> io::Result<(u64, u64, u64)> {
        unsafe {
            let mut available_free_space: ULARGE_INTEGER = std::mem::zeroed();
            let mut total_size: ULARGE_INTEGER = std::mem::zeroed();
            let mut total_free_space: ULARGE_INTEGER = std::mem::zeroed();
            let result = winapi::um::fileapi::GetDiskFreeSpaceExW(
                self.encoded_root_path.as_ptr(),
                &mut available_free_space,
                &mut total_size,
                &mut total_free_space,
            );
            if 0 == result {
                return Err(io::Error::last_os_error());
            } else {
                return Ok((
                    *available_free_space.QuadPart(),
                    *total_size.QuadPart(),
                    *total_free_space.QuadPart(),
                ));
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
pub fn get_drives() -> Vec<DriveInfo> {
    let mut result = vec![];
    for drive_letter in get_drive_labels() {
        let drive_name = drive_letter.to_string();
        let drive_info = DriveInfo::new(drive_name);
        result.push(drive_info);
    }
    result
}
