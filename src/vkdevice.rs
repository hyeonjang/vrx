extern crate ash;
extern crate winit;

use ash::extensions::{
    ext::DebugUtils,
    khr::{Surface, Swapchain},
};
use ash::{vk};
use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::c_char;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Vkdevice {
    pub entry: ash::Entry,
    pub device: ash::Device,
    pub instance: ash::Instance,
}

impl Vkdevice {
    fn create_instance(entry:&ash::Entry) -> ash::Instance {
        unsafe {
        // 1. application initialize
        let app_name = CStr::from_bytes_with_nul_unchecked(b"vk-cholesky\0");
        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .engine_name(app_name);

        // 
        let extension_propertices = entry.enumerate_instance_extension_properties(None);

        // 2. instance initialize 
        let instance_create_flag = vk::InstanceCreateFlags::default();
        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info).flags(instance_create_flag).build();

        // 
        let instance = entry.create_instance(&instance_create_info, None).unwrap();
        instance
        }
    }
    
    pub fn new() {
        unsafe {
            let entry = ash::Entry::load().unwrap();
            let instance = Vkdevice::create_instance(&entry);
        }
    }
}
pub struct Compute {
    pub queue : vk::Queue,
    pub cmd_pool : vk::CommandPool,
    pub cmd_buffer : vk::CommandBuffer,
    pub semaphore : vk::Semaphore,
    pub desc_set_layout : vk::DescriptorSetLayout,
    pub desc_set : vk::DescriptorSet,
    pub pipeline_layout : vk::PipelineLayout,
    pub pipeline : vk::Pipeline,
}

impl Compute {

    pub fn buffer() {
        let cmd_buf_info = vk::CommandBufferBeginInfo::default();



    }
}