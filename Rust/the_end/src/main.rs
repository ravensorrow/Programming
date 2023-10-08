extern crate winapi;

use std::ptr::null_mut as NULL;
use winapi::um::winuser;

fn main() {
    let l_msg: Vec<u16> = "The End is nigh!\0".encode_utf16().collect();
    let l_title: Vec<u16> = "GrunksWorld!\0".encode_utf16().collect();

    unsafe {
        winuser::MessageBoxW(NULL(), l_msg.as_ptr(), l_title.as_ptr(), winuser::MB_OK | winuser::MB_ICONINFORMATION);
    }
}
