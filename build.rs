extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn compile_shader() {
    let args = [
        "./src/shader/cholesky.comp",
        "-o",
        "./src/shader/cholesky.spv",
    ];
    Command::new("glslc").args(&args).output().expect("failed");

    //@@ here cannot catch bug
}

fn main() {
    // check env
    let vulkan_sdk = env::var("VULKAN_SDK").unwrap();
    let vulkan_sdk = Path::new(&vulkan_sdk);
    // link
    if cfg!(unix) {
        println!(
            "cargo:rustc-link-search={}",
            vulkan_sdk.join("lib/").to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=vulkan");
    } else if cfg!(windows) {
        println!(
            "cargo:rustc-link-search={}",
            vulkan_sdk.join("Lib/").to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=vulkan-1");
    }

    // rust header bindings
    let bindings = bindgen::Builder::default()
        .header(vulkan_sdk.join("include/vulkan/vulkan.h").to_str().unwrap())
        .prepend_enum_name(false)
        .derive_default(true)
        .size_t_is_usize(true)
        .rustified_enum("VkResult")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("vulkan_header.rs"))
        .expect("Couldn't write bindings!");

    // spv generation
    compile_shader();
    println!("cargo:rerun-if-changed=src/shader/cholesky.comp");
}
