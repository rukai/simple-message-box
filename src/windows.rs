extern crate winapi;

use self::winapi::um::winuser::{MessageBoxW, MB_OK};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

pub(crate) fn create_message_box_impl(message: &str, title: &str) {
    let message_wide: Vec<u16> = OsStr::new(message).encode_wide().chain(once(0)).collect();
    let title_wide:   Vec<u16> = OsStr::new(title  ).encode_wide().chain(once(0)).collect();
    let ret = unsafe { MessageBoxW(null_mut(), message_wide.as_ptr(), title_wide.as_ptr(), MB_OK) };
    if ret == 0 {
        eprintln!("Failed to create error message window");
    }
}
