use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    let make_status = Command::new("make")
        .current_dir("ssd1306_linux")
        .status()
        .expect("Failed to execute make command");

    if !make_status.success() {
        panic!("make command failed in ssd1306_linux directory");
    }

    let source_path = Path::new("ssd1306_linux/libssd1306.so");
    let dest_path = Path::new("libs/libssd1306.so");

    if !source_path.exists() {
        panic!("The compiled library libssd1306.so was not found in ssd1306_linux");
    }

    if let Err(e) = fs::create_dir_all("libs") {
        panic!("Failed to create libs directory: {}", e);
    }

    if let Err(e) = fs::copy(&source_path, &dest_path) {
        panic!(
            "Failed to copy libssd1306.so from {:?} to {:?}: {}",
            source_path, dest_path, e
        );
    }

    println!("cargo:rustc-link-search=native=libs");
    println!("cargo:rustc-link-lib=ssd1306");
}
