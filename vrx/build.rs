extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // check env
    let vulkan_sdk = env::var("VULKAN_SDK").expect("[vrx] No vulkan enviroment. Please check");
    let vulkan_sdk = Path::new(&vulkan_sdk);

    // platform specific options
    // linux
    if cfg!(unix) {
        println!(
            "cargo:rustc-link-search={}",
            vulkan_sdk.join("lib/").to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=vulkan");
    // windows
    // issue: https://github.com/rust-lang/rust-bindgen/issues/1556
    } else if cfg!(windows) {
        println!(
            "cargo:rustc-link-search={}",
            vulkan_sdk.join("Lib/").to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=vulkan-1");
    }

    // check built header
    if cfg!(feature = "graphics") {
        let check_graphics = PathBuf::from("./src/vk_graphics_header.rs");
        if check_graphics.exists() {
            return;
        }
    } else if cfg!(feature = "computes") {
        let check_computes = PathBuf::from("./src/vk_computes_header.rs");
        if check_computes.exists() {
            return;
        }
    }

    // bindgen
    let mut bind_builder = bindgen::Builder::default()
        .header(vulkan_sdk.join("include/vulkan/vulkan.h").to_str().unwrap())
        .prepend_enum_name(false)
        .derive_default(true)
        .no_default("[^V]*")
        .size_t_is_usize(true)
        .rustified_enum("VkResult");

    if cfg!(windows) {
        if cfg!(feature = "graphics") {
            bind_builder = bind_builder
                .clang_arg("-DVK_USE_PLATFORM_WIN32_KHR")
                .blocklist_type("LPMONITORINFOEXA?W?")
                .blocklist_type("LPTOP_LEVEL_EXCEPTION_FILTER")
                .blocklist_type("MONITORINFOEXA?W?")
                .blocklist_type("PEXCEPTION_FILTER")
                .blocklist_type("PEXCEPTION_ROUTINE")
                .blocklist_type("PSLIST_HEADER")
                .blocklist_type("PTOP_LEVEL_EXCEPTION_FILTER")
                .blocklist_type("PVECTORED_EXCEPTION_HANDLER")
                .blocklist_type("_?L?P?CONTEXT")
                .blocklist_type("_?L?P?EXCEPTION_POINTERS")
                .blocklist_type("_?P?DISPATCHER_CONTEXT")
                .blocklist_type("_?P?EXCEPTION_REGISTRATION_RECORD")
                .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
                .blocklist_type("_?P?NT_TIB")
                .blocklist_type("tagMONITORINFOEXA")
                .blocklist_type("tagMONITORINFOEXW")
                .blocklist_function("AddVectoredContinueHandler")
                .blocklist_function("AddVectoredExceptionHandler")
                .blocklist_function("CopyContext")
                .blocklist_function("GetThreadContext")
                .blocklist_function("GetXStateFeaturesMask")
                .blocklist_function("InitializeContext")
                .blocklist_function("InitializeContext2")
                .blocklist_function("InitializeSListHead")
                .blocklist_function("InterlockedFlushSList")
                .blocklist_function("InterlockedPopEntrySList")
                .blocklist_function("InterlockedPushEntrySList")
                .blocklist_function("InterlockedPushListSListEx")
                .blocklist_function("LocateXStateFeature")
                .blocklist_function("QueryDepthSList")
                .blocklist_function("RaiseFailFastException")
                .blocklist_function("RtlCaptureContext")
                .blocklist_function("RtlCaptureContext2")
                .blocklist_function("RtlFirstEntrySList")
                .blocklist_function("RtlInitializeSListHead")
                .blocklist_function("RtlInterlockedFlushSList")
                .blocklist_function("RtlInterlockedPopEntrySList")
                .blocklist_function("RtlInterlockedPushEntrySList")
                .blocklist_function("RtlInterlockedPushListSListEx")
                .blocklist_function("RtlQueryDepthSList")
                .blocklist_function("RtlRestoreContext")
                .blocklist_function("RtlUnwindEx")
                .blocklist_function("RtlVirtualUnwind")
                .blocklist_function("SetThreadContext")
                .blocklist_function("SetUnhandledExceptionFilter")
                .blocklist_function("SetXStateFeaturesMask")
                .blocklist_function("UnhandledExceptionFilter")
                .blocklist_function("__C_specific_handler");
        }
    }

    let out_path = PathBuf::from("./src");
    // rust header bindings
    if cfg!(feature = "graphics") {
        bind_builder
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_path.join("vk_graphics_header.rs"))
            .expect("Couldn't write bindings!");
    } else if cfg!(feature = "computes") {
        bind_builder
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_path.join("vk_computes_header.rs"))
            .expect("Couldn't write bindings!");
    }
}
