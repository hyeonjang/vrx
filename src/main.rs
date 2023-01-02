use vkcholesky::*;

use std::ptr::{null};

use crate::vx::c_void;
fn main() {
    // let device = vx::new_compute_device();

    // let info = VkBufferCreateInfo {
    //     sType:VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
    //     flags:VK_BUFFER_CREATE_PROTECTED_BIT,
    //     size:65536,        
    //     usage:VK_BUFFER_USAGE_TRANSFER_SRC_BIT,
    //     sharingMode:VK_SHARING_MODE_CONCURRENT,
    //     queueFamilyIndexCount:0,
    //     pQueueFamilyIndices:null(),
    //     pNext:null(),
    // };

    // let mut state = 20;
    // let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
    // unsafe {
    //     let buf = device.create_buffer(info, 0, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, state_ptr);
    // }
}
