use i_drive::{screen_clear, screen_init, screen_write};
use std::ffi::CString;
use std::fs;
use std::io;
use std::thread::sleep;
use std::time::Duration;

const NODE_ADDRESS: u8 = 1;

fn main() {
    screen_init(NODE_ADDRESS);

    loop {
        screen_clear();

        match get_temp() {
            Ok(temp) => {
                let temp_string = CString::new(temp).expect("couldn't put cstring on heap");
                let temp_string_ptr = temp_string.as_ptr() as *mut u8;
                screen_write(temp_string_ptr);
            }
            Err(err) => panic!("{}", err),
        }

        sleep(Duration::from_secs(1));
    }
}

fn get_temp() -> Result<String, io::Error> {
    let temp_path = "/sys/class/thermal/thermal_zone0/temp";
    match fs::read_to_string(temp_path) {
        Ok(temp_str) => {
            if let Ok(temp) = temp_str.trim().parse::<i32>() {
                let cpu_temp_c = temp / 1000;
                Ok(format!("{}", cpu_temp_c))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unable to parse temperature",
                ))
            }
        }
        Err(err) => Err(err),
    }
}
