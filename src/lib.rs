#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// mod bindings;
// pub use bindings::*;

include!("bindings.rs");

use std::ptr::*;
use std::ffi::*;

macro_rules! VK_MAKE_VERSION {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

use paste::paste;

macro_rules! vk_instantiate {
    ( $x:ident ) => {
        {
            paste! {
                let mut type_T = [<$x _T>] { _unused : [0;0] };
                let mut type_inst : *mut [<$x _T>] = &mut type_T;
            }

            type_inst
        }
    };
}

pub struct Context {
    instance : VkInstance,
    physical_devices : Vec<VkPhysicalDevice>,
}

impl Context {
    pub fn new() -> Context {

        let mut instance = vk_instantiate!(VkInstance);
        let mut physical_devices = vec![];

        unsafe {
            // instance
            {
                let app_info = VkApplicationInfo {
                    sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                    pNext: null(),
                    pApplicationName: CString::new("vkcholesky").unwrap().as_ptr(),
                    applicationVersion: 1,
                    pEngineName: CString::new("No engine").unwrap().as_ptr(),
                    engineVersion: 1,
                    apiVersion: 1,
                };              
                
                let layers = CString::new("VK_LAYER_KHRONOS_validation");
                let extensions = CString::new("VK_EXT_debug_report");

                let instance_create_info = VkInstanceCreateInfo {
                    sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                    pNext: null(),
                    flags: 0,
                    pApplicationInfo:  &app_info,
                    enabledLayerCount: 1,
                    ppEnabledLayerNames: &layers.unwrap().as_ptr(),
                    enabledExtensionCount: 1,
                    ppEnabledExtensionNames: &extensions.unwrap().as_ptr(),
                };

                vkCreateInstance(&instance_create_info, null(), &mut instance);

                println!("{:?}", instance);
                println!("{:?}", &instance);
                println!("create instance");

                let mut device_count = 0 as u32;
                println!("{:?}", device_count);

                vkEnumeratePhysicalDevices(instance, &mut device_count, null_mut());
                
                println!("{:?}", device_count);

                let mut physical_devices = vec![null_mut(); device_count as usize];
                vkEnumeratePhysicalDevices(instance, &mut device_count, physical_devices.as_mut_ptr());

            }
        }

        Context { instance:instance as VkInstance, physical_devices:physical_devices } 

    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn t_context() {
        let ctx = Context::new();
    }

}