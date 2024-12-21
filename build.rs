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

    let bindings = bindgen::Builder::default()
        .header("ssd1306_linux/ssd1306.h")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = Path::new("src/ssd1306_bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=ssd1306_linux/ssd1306.h");
    println!("cargo:rustc-link-search=native=libs");
    println!("cargo:rustc-link-lib=ssd1306");
}
