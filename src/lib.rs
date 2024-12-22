mod ssd1306_bindings;

use ssd1306_bindings::{
    ssd1306_end, ssd1306_init, ssd1306_oled_clear_screen, ssd1306_oled_default_config,
    ssd1306_oled_set_XY, ssd1306_oled_set_rotate, ssd1306_oled_write_string,
};

pub use ssd1306_bindings::{SSD1306_FONT_NORMAL, SSD1306_FONT_SMALL};

pub fn screen_init(node_address: u8) -> u8 {
    unsafe {
        let mut rc = ssd1306_init(node_address);

        if rc != 0 {
            panic!("no ssd1306 chipset on /dev/i2c-{}", node_address);
        }

        rc += ssd1306_oled_default_config(48, 64);
        rc += ssd1306_oled_clear_screen();
        rc += ssd1306_oled_set_rotate(0);
        rc += ssd1306_oled_set_XY(1, 1);

        println!("rc: {}", rc);

        rc
    }
}

pub fn screen_write(c_string_ptr: *mut u8) -> u8 {
    unsafe {
        let result = ssd1306_oled_write_string(SSD1306_FONT_NORMAL as u8, c_string_ptr);
        println!("Result: {}", result);

        result
    }
}

pub fn screen_clear() {
    unsafe {
        ssd1306_oled_clear_screen();
        ssd1306_oled_set_XY(1, 1);
    }
}

pub fn screen_end() {
    unsafe {
        ssd1306_end();
    }
}
