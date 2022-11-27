#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // add vk-cholesky
    println!("cargo:rustc-link-search=C:/VulkanSDK/1.3.216.0/Lib");
    println!("cargo:rustc-link-search=build/Debug/");
    println!("cargo:rustc-link-lib=vulkan-1");
    println!("cargo:rustc-link-lib=vkcholesky");

    // rebuild
    println!("cargo:rerun-if-changed=src/vkcholesky.hpp");

    let bindings = bindgen::Builder::default()
        .header("src/vkcholesky.hpp")
        .clang_arg("-IC:/VulkanSDK/1.3.216.0/Include")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    // let out_path = let::from(env::var("C:/Users/hyeon/bindgen-tutorial-bzip2-sys").unwrap());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
