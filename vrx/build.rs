extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // check env
    let vulkan_sdk = env::var("VULKAN_SDK").expect("[vrx] No vulkan enviroment. Please check");
    let vulkan_sdk = Path::new(&vulkan_sdk);

    println!("cargo:rerun-if-changed=build.rs");
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
    // if cfg!(feature = "graphics") {
    //     let check_graphics = PathBuf::from("./src/vk_graphics_header.rs");
    //     if check_graphics.exists() {
    //         return;
    //     }
    // } else if cfg!(feature = "computes") {
    //     let check_computes = PathBuf::from("./src/vk_computes_header.rs");
    //     if check_computes.exists() {
    //         return;
    //     }
    // }

    // bindgen
    let mut bind_builder = bindgen::Builder::default()
        .header(vulkan_sdk.join("include/vulkan/vulkan.h").to_str().unwrap())
        .prepend_enum_name(false)
        .derive_default(true)
        .no_default("[^V]*")
        .size_t_is_usize(true)
        //
        // Vulkan C type macro to rust enum
        //
        .newtype_enum("VkImageLayout")
        .newtype_enum("VkResult")
        .newtype_enum("VkFormat")
        .newtype_enum("VkImageTiling")
        .newtype_enum("VkImageType")
        .newtype_enum("VkPhysicalDeviceType")
        .newtype_enum("VkQueryType")
        .newtype_enum("VkSharingMode")
        .newtype_enum("VkComponentSwizzle")
        .newtype_enum("VkImageViewType")
        .newtype_enum("VkBlendFactor")
        .newtype_enum("VkBlendOp")
        .newtype_enum("VkCompareOp")
        .newtype_enum("VkDynamicState")
        .newtype_enum("VkFrontFace")
        .newtype_enum("VkVertexInputRate")
        .newtype_enum("VkPrimitiveTopology")
        .newtype_enum("VkPolygonMode")
        .newtype_enum("VkStencilOp")
        .newtype_enum("VkLogicOp")
        .newtype_enum("VkBorderColor")
        .newtype_enum("VkFilter")
        .newtype_enum("VkSamplerAddressMode")
        .newtype_enum("VkSamplerMipmapMode")
        .newtype_enum("VkDescriptorType")
        .newtype_enum("VkAttachmentLoadOp")
        .newtype_enum("VkAttachmentStoreOp")
        .newtype_enum("VkPipelineBindPoint")
        .newtype_enum("VkCommandBufferLevel")
        .newtype_enum("VkIndexType")
        .newtype_enum("VkSubpassContents");
    //
    // to interoperate between flag and flagbits
    // can be invalid according to the official bindgen doc
    // "should be other method"
    //
    // .rustified_enum("VkAccessFlagBits")
    // .rustified_enum("VkImageAspectBits")
    // .rustified_enum("VkFormatFeatureBits")
    // .rustified_enum("VkImageCreateFlagBits")
    // .rustified_enum("VkSampleCountFlagBits")
    // .rustified_enum("VkImageUsageFlagBits")
    // .rustified_enum("VkInstanceCreateFlagBits")
    // .rustified_enum("VkMemoryHeapFlagBits")
    // .rustified_enum("VkMemoryPropertyFlagBits")
    // .rustified_enum("VkQueueFlagBits")
    // .rustified_enum("VkDeviceQueueCreateFlagBits")
    // .rustified_enum("VkPipelineStageFlagBits")
    // .rustified_enum("VkSparseMemoryBindFlagBits")
    // .rustified_enum("VkSparseImageFormatFlagBits")
    // .rustified_enum("VkFenceCreateFlagBits")
    // .rustified_enum("VkEventCreateFlagBits")
    // .rustified_enum("VkQueryPipelineStatisticFlagBits")
    // .rustified_enum("VkQueryPoolCreateFlagBits")
    // .rustified_enum("VkQueryPolCreateFlagBits")
    // .rustified_enum("VkBufferCreateFlagBits")
    // .constified_enum("VkBufferUsageFlagBits")
    // .rustified_enum("VkImageViewCreateFlagBits")
    // .rustified_enum("VkPipelineCacheCreateFlagBits")
    // .rustified_enum("VkColorComponentFlagBits")
    // .rustified_enum("VkPipelineCreateFlagBits")
    // .rustified_enum("VkPipelineShaderStageCreateFlagBits")
    // .rustified_enum("VkShaderStageFlagBits")
    // .rustified_enum("VkCullModeFlagBits")
    // .rustified_enum("VkPipelineDepthStencilStateCreateFlagBits")
    // .rustified_enum("VkPipelineColorBlendSateCreateFlagBits")
    // .rustified_enum("VkPipelineLayoutCreateFlagBits")
    // .rustified_enum("VkSamplerCreateFlagBits")
    // .rustified_enum("VkDescriptorPoolCreateFlagBits")
    // .rustified_enum("VkDescriptorSetLayoutCreateFlagBits")
    // .rustified_enum("VkAttachmentDescriptionFlagBits")
    // .rustified_enum("VkDependencyFlagBits")
    // .rustified_enum("VkFramebufferCreateFlagBits")
    // .rustified_enum("VkRenderPassCreateFlagBits")
    // .rustified_enum("VkSubpassDescriptionFlagBits")
    // .rustified_enum("VkCommandPoolCreateFlagBits")
    // .rustified_enum("VkCommandBufferUsageFlagBits")
    // .rustified_enum("VkQueryControlFlagBits")
    // .rustified_enum("VkCommandBufferResetFlagBits")
    // .rustified_enum("VkStencilFaceFlagBits");

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
                .blocklist_type("_?L?DISPATCHER_CONTEXT_ARM64")
                .blocklist_type("_?P?DISPATCHER_CONTEXT_ARM64")
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
