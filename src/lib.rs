#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod bindings;
pub use bindings::*;

// cxx
use cxx::*;
unsafe impl ExternType for bindings::VkBufferCreateInfo {
    type Id = type_id!("VkBufferCreateInfo");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
pub mod vx {

    unsafe extern "C++" {
        include!("vkcholesky/src/vkcholesky.h");

        // Vulkan type
        type VkBufferCreateInfo = crate::bindings::VkBufferCreateInfo;

        // Custom type
        type Context;
        fn vulkan_context() -> UniquePtr<Context>;

        type Device;

        fn new_compute_device() -> UniquePtr<Device>;
        fn create_buffer(self:&Device, info:VkBufferCreateInfo, size:usize) -> UniquePtr<Buffer>;
        // #[Self="Device"]
        // fn new_compute_device(self:&Device) -> UniquePtr<Device>;
        type Buffer;

        type CommandBuffer;
        fn begin();
        fn end();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn t_vulkan() {
        ffi::initVulkan();
    }
}