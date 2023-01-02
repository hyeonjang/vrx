#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod bindings;
pub use bindings::*;

use std::ptr::*;

struct Context {
    instance : VkInstance,
    physical_devices : Vec<VkPhysicalDevice>,
}

impl Context {
    fn new(&self) -> Context {
        unsafe {
            // instance
            {
                let app_info = VkApplicationInfo {
                    sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                    pNext: null(),
                    pApplicationName: "vkcholesky",
                    applicationVersion: todo!(),
                    pEngineName: "No Engine" as *const i8,
                    engineVersion: todo!(),
                    apiVersion: VK_VERSION_1_2,
                };                
            }
        }
    }
}

// cxx
// use cxx::*;


// unsafe impl ExternType for bindings::VkBufferCreateInfo {
//     type Id = type_id!("VkBufferCreateInfo");
//     type Kind = cxx::kind::Trivial;
// }

// unsafe impl ExternType for bindings::VkDescriptorBufferInfo {
//     type Id = type_id!("VkDescriptorBufferInfo");
//     type Kind = cxx::kind::Trivial;
// }

// unsafe impl ExternType for bindings::VkBuffer_T {
//     type Id = type_id!("VkBuffer_T");
//     type Kind = cxx::kind::Trivial;
// }

// unsafe impl ExternType for bindings::VkDeviceMemory_T {
//     type Id = type_id!("VkDeviceMemory_T");
//     type Kind = cxx::kind::Trivial;
// }

// #[cxx::bridge]
// pub mod vx {

//     unsafe extern "C++" {
//         include!("vkcholesky/src/vkcholesky.h");

//         // std type
//         type c_void;

//         // Vulkan type
//         // type VkMemoryPropertyFlags = u32;
//         type VkBufferCreateInfo = crate::bindings::VkBufferCreateInfo;
//         type VkDescriptorBufferInfo = crate::bindings::VkDescriptorBufferInfo;

//         // Custom type
//         type Context;
//         fn vulkan_context() -> UniquePtr<Context>;

//         type Device;
//         fn new_compute_device() -> UniquePtr<Device>;
//         unsafe fn create_buffer(self:&Device, info:VkBufferCreateInfo, size:usize, flag:u32, data:*mut c_void) -> UniquePtr<Buffer>;
//         // #[Self="Device"]
//         // fn new_compute_device(self:&Device) -> UniquePtr<Device>;
//         type Buffer;
//         // fn alloc(self:&Buffer, flags:VkMemoryPropertyFlags);
//         // unsafe fn map(self:&Buffer, data:*mut c_void);

//         type Descriptor;
//         fn update(self:&Descriptor, info:VkDescriptorBufferInfo, index:usize);

//         type CommandBuffer;
//         fn begin(self:&CommandBuffer);
//         fn end(self:&CommandBuffer);
//     }
// }



#[cfg(test)]
mod tests {

    use super::*;

}