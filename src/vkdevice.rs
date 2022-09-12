extern crate ash;
extern crate winit;

use ash::extensions::{
    ext::DebugUtils,
    khr::{Surface, Swapchain},
};
use ash::vk;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct VkContext {
    entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: usize,
}

impl VkContext {
    pub fn new() -> VkContext {
        unsafe {
            let ash_entry = ash::Entry::load().unwrap();
            let ash_instance = VkContext::create_instance(&ash_entry);
            let (vk_physical_device, vk_queue_family_index) =
                VkContext::create_physical_device(&ash_instance);
            // let vk_device =
            // VkContext::create_device(&ash_instance, &vk_physical_device, vk_queue_family_index);
            VkContext {
                entry: (ash_entry),
                instance: (ash_instance),
                physical_device: (vk_physical_device),
                queue_family_index: (vk_queue_family_index),
            }
        }
    }

    pub fn device(&self) -> ash::Device {
        unsafe {
            VkContext::create_device(
                &self.instance,
                &self.physical_device,
                self.queue_family_index,
            )
        }
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        unsafe {
            // 1. application initialize
            let app_name = CStr::from_bytes_with_nul_unchecked(b"vk-cholesky\0");
            let app_info = vk::ApplicationInfo::builder()
                .application_name(app_name)
                .engine_name(app_name);

            //
            let layer_names = [CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )];
            let layers_names_raw: Vec<*const c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let extension_propertices = entry.enumerate_instance_extension_properties(None);
            let extension_names = [DebugUtils::name().as_ptr()];

            // 2. instance initialize
            let instance_create_flag = vk::InstanceCreateFlags::default();
            let instance_create_info = vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names)
                .flags(instance_create_flag)
                .build();

            //
            let instance = entry.create_instance(&instance_create_info, None).unwrap();
            instance
        }
    }

    fn create_physical_device(instance: &ash::Instance) -> (vk::PhysicalDevice, usize) {
        unsafe {
            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("[Vkdevice] physcial device error");

            let (physical_device, queue_family_index) = physical_devices
                .iter()
                .find_map(|physical_device| {
                    instance
                        .get_physical_device_queue_family_properties(*physical_device)
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            let supports_compute =
                                info.queue_flags.contains(vk::QueueFlags::COMPUTE);
                            if supports_compute {
                                Some((*physical_device, index))
                            } else {
                                None
                            }
                        })
                })
                .expect("[Vkdevice] No suitable device");

            (physical_device, queue_family_index)
        }
    }

    fn create_device(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        qfam_index: usize,
    ) -> ash::Device {
        unsafe {
            let priorities = [1.0];

            let device_queue_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(qfam_index as u32)
                .queue_priorities(&priorities);

            let device_extension_names = [
                // ash::extensions::khr::Swapchain::name().as_ptr(),
                // ash::vk::KhrPortabilityEnumerationFn::name().as_ptr(),
            ];

            let physical_device_features = vk::PhysicalDeviceFeatures::default();

            let device_create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(std::slice::from_ref(&device_queue_info))
                .enabled_extension_names(&device_extension_names)
                .enabled_features(&physical_device_features);

            let device = instance
                .create_device(*physical_device, &device_create_info, None)
                .unwrap();

            device
        }
    }
}
pub struct VkCompute {
    pub queue: vk::Queue,
    pub cmd_pool: vk::CommandPool,
    pub cmd_buffer: vk::CommandBuffer,
    pub semaphore: vk::Semaphore,
    pub desc_set_layout: vk::DescriptorSetLayout,
    pub desc_set: vk::DescriptorSet,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl VkCompute {
    pub fn new(device:&ash::Device) {
        unsafe {
        // let vk_queue = // device.get_device_queue(queue_family_index, queue_index)

        // device.create_descriptor_set_layout(create_info, allocation_callbacks)
        let mut desc_set_layout_bindings;
        {
            let desc_set_layout_binding0 = vk::DescriptorSetLayoutBinding::builder()
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .binding(0).build();

            let desc_set_layout_binding1 = vk::DescriptorSetLayoutBinding::builder()
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .binding(1).build();

            desc_set_layout_bindings = [desc_set_layout_binding0, desc_set_layout_binding1];
        };
        let desc_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(&desc_set_layout_bindings);
        let mut desc_set_layouts;
        {
            let desc_set_layout = device.create_descriptor_set_layout(&desc_set_layout_create_info, None).unwrap();
            desc_set_layouts = [desc_set_layout];
        }

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::builder().set_layouts(&desc_set_layouts);
        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_create_info, None).unwrap();

        let mut compute_pipeline_create_infos;
        {
            // let mut spv_file = Cursor::new(&);
            // let code = ash::util::read_spv("./shader/cholesky.spv").unwrap();
            // vk::ShaderModuleCreateInfo::builder().code(&code);
            // let shader_module = device.create_shader_module(create_info, allocation_callbacks);

            let compute_pipeline_create_info = vk::ComputePipelineCreateInfo::builder()
                .layout(pipeline_layout).build();
                // .stage(stage).build();
            

            compute_pipeline_create_infos = [compute_pipeline_create_info];
        }
        
        let vk_pipeline_cache = vk::PipelineCache::default();
        let compute_pipeline = device.create_compute_pipelines(vk_pipeline_cache, &compute_pipeline_create_infos, None);
        }
    }

    fn create_desc_set_layouts() {

    }
}
