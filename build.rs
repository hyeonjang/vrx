#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake::Config;

// 0. specify vulkan path by OS dependently
// 1. from ./build path to absolute

fn main() {
    
    // check vulkan
    // Vulkan_SDK
    // VK_LAYER_PATH
    if cfg!(target_os = "windows") {
        
    }

    let dst = Config::new("./").build().join("build");
    // add vk-cholesky
    // println!("cargo:rustc-link-search=C:/VulkanSDK/1.3.216.0/Lib");
    println!("cargo:rustc-link-search=/home/hyeonjang/vulkan/1.3.204.1/x86_64/lib");
    println!("cargo:rustc-link-search={}", dst.display());
    // println!("cargo:rustc-link-lib=vulkan-1");
    println!("cargo:rustc-link-lib=vulkan");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", dst.display());
    println!("cargo:rustc-link-lib=dylib=vkcholesky");

    // rebuild
    println!("cargo:rerun-if-changed=src/vkcholesky.hpp");
    println!("cargo:rerun-if-changed=src/vkcholesky.cpp");

    let bindings = bindgen::Builder::default()
        .header("src/vkcholesky.hpp")
        // .clang_arg("-IC:/VulkanSDK/1.3.216.0/Include")
        .clang_arg("-I/home/hyeonjang/vulkan/1.3.204.1/x86_64/include")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}
