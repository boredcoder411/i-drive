use i_drive::{screen_init, screen_write, screen_clear, screen_end};
use std::ffi::CString;
use std::thread::sleep;
use std::time::Duration;

const NODE_ADDRESS: u8 = 1;

fn main() {
    screen_init(NODE_ADDRESS);

    let c_string = CString::new("Hello, OLED!").expect("CString::new failed");
    let c_string_ptr = c_string.as_ptr() as *mut u8;

    screen_write(c_string_ptr);

    sleep(Duration::from_secs(1));

    screen_clear();

    screen_end();
}