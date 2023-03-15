use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::LPARAM;
use winapi::um::{
    winuser::{FindWindowExW, SendMessageW, COPYDATASTRUCT, WM_COPYDATA},
};

pub fn send_now_playing(format: &str, artist: &str, title: &str, album: &str) {
    let msg = format!("\\0Music\\01\\0{}\\0{}\\0{}\\0{}\\0WMContentID\\0", format, artist, title, album);
    let wide_msg: Vec<u16> = OsStr::new(&msg).encode_wide().chain(Some(0)).collect();

    let mut copy_data = COPYDATASTRUCT {
        dwData: 0x0547,
        cbData: (wide_msg.len() * 2 + 2) as u32,
        lpData: wide_msg.as_ptr() as *mut _,
    };

    let window_title: Vec<u16> = OsString::from("MsnMsgrUiManager").encode_wide().chain(Some(0)).collect();
    let mut hwnd = std::ptr::null_mut();
    loop {
        hwnd = unsafe { FindWindowExW(std::ptr::null_mut(), hwnd, window_title.as_ptr(), std::ptr::null()) };
        if hwnd.is_null() {
            break;
        }
        unsafe { SendMessageW(hwnd, WM_COPYDATA, 0, &mut copy_data as *mut _ as LPARAM) };
    }
}
