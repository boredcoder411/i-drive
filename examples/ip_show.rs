use i_drive::{screen_clear, screen_init, screen_write};
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::io;
use std::thread::sleep;
use std::time::Duration;

const NODE_ADDRESS: u8 = 1;

fn main() {
    screen_init(NODE_ADDRESS);

    loop {
        screen_clear();

        let ip_string = CString::new(get_ip()).expect("failed to allocate c string on heap");
        let ip_string_ptr = ip_string.as_ptr() as *mut u8;

        screen_write(ip_string_ptr);

        sleep(Duration::from_secs(1));
    }
}

fn get_ip() -> String {
        // Execute the command pipeline step by step
    let ifconfig_output = Command::new("ifconfig")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute `ifconfig`");

    let grep_inet = Command::new("grep")
        .arg("-Eo")
        .arg("inet (addr:)?([0-9]*\\.){3}[0-9]*")
        .stdin(ifconfig_output.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute `grep -Eo 'inet ...'`");

    let grep_ip = Command::new("grep")
        .arg("-Eo")
        .arg("([0-9]*\\.){3}[0-9]*")
        .stdin(grep_inet.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute `grep -Eo '([0-9]*\\.)...'`");

    let grep_exclude_localhost = Command::new("grep")
        .arg("-v")
        .arg("127.0.0.1")
        .stdin(grep_ip.stdout.unwrap())
        .output()
        .expect("Failed to execute `grep -v '127.0.0.1'`");

    // Print the final output
    let result = String::from_utf8_lossy(&grep_exclude_localhost.stdout);

    // return the string
    result.to_string().trim().to_string()
}
