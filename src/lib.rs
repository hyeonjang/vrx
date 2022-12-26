#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod bindings;
use cxx::{type_id, ExternType};

unsafe impl ExternType for bindings::VkInstance_T {
    type Id = type_id!("VkInstance_T");
    type Kind = cxx::kind::Opaque;
}


#[cxx::bridge]
pub mod vx {

    unsafe extern "C++" {
        include!("vkcholesky/src/vkcholesky.h");

        // Vulkan type
        type VkInstance_T = crate::bindings::VkInstance_T;
        type VkDevice;
        type VkBuffer;
        type VkBufferCreateInfo;

        // Custom type
        type Context;

        type Device;
        fn new_compute_device(self:&Device) -> UniquePtr<Device>;

        type Buffer;
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