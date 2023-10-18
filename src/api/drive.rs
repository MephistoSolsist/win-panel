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
