mod ssd1306_bindings;

use ssd1306_bindings::{
    ssd1306_end, ssd1306_init, ssd1306_oled_clear_screen, ssd1306_oled_default_config,
    ssd1306_oled_set_XY, SSD1306_FONT_NORMAL,
};
use std::ffi::CString;

const NODE_ADDRESS: u8 = 1;

fn main() {
    unsafe {
        let mut rc = ssd1306_init(NODE_ADDRESS);

        if rc != 0 {
            panic!("no ssd1306 chipset on /dev/i2c-{}", NODE_ADDRESS);
        }

        rc += ssd1306_oled_default_config(48, 64);

        rc += ssd1306_oled_clear_screen();

        rc += ssd1306_bindings::ssd1306_oled_set_rotate(0);

        rc += ssd1306_oled_set_XY(1, 7);

        println!("rc: {}", rc);
    }

    let c_string = CString::new("Hello, OLED!").expect("CString::new failed");
    let c_string_ptr = c_string.as_ptr() as *mut u8;

    unsafe {
        let result = ssd1306_bindings::ssd1306_oled_write_string(SSD1306_FONT_NORMAL as u8, c_string_ptr);
        println!("Result: {}", result);

        ssd1306_end();
    }
}