use vkcholesky::*;

use std::ptr::{null};

fn main() {
    let device = vx::new_compute_device();

    let info = VkBufferCreateInfo {
        sType:VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        flags:VK_BUFFER_CREATE_PROTECTED_BIT,
        size:65536,        
        usage:VK_BUFFER_USAGE_TRANSFER_SRC_BIT,
        sharingMode:VK_SHARING_MODE_CONCURRENT,
        queueFamilyIndexCount:0,
        pQueueFamilyIndices:null(),
        pNext:null(),
    };

    let buf = device.create_buffer(info, 0);
}
