extern crate winapi;

fn main() {
    unsafe {
        let mut drives = winapi::um::fileapi::GetLogicalDrives();
        for drive_letter in b'A'..=b'Z' {
            if (drives & 1) != 0 {
                println!("Drive {} is available.", drive_letter as char);
            }
            drives >>= 1;
        }
    }
}
