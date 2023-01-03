#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("bindings.rs");
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ptr::*;
use std::ffi::*;
use std::str::*;
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

macro_rules! vk_default {
    ( $x:ident ) => {
        impl Default for $x {
            fn default() -> $x {
                $x { _unused : [0;0] }
            }
        }
    };
}

#[macro_export]
macro_rules! vk_instantiate {
    ( $x:ident ) => {
        {
            use paste::paste;

            paste! {
                let mut type_T = [<$x _T>]::default();
                let mut type_inst : *mut [<$x _T>] = &mut type_T;
            }

            type_inst
        }
    };
}

vk_default!(VkBuffer_T);
vk_default!(VkImage_T);
vk_default!(VkInstance_T);
vk_default!(VkPhysicalDevice_T);
vk_default!(VkDevice_T);
vk_default!(VkQueue_T);
vk_default!(VkSemaphore_T);
vk_default!(VkCommandBuffer_T);
vk_default!(VkFence_T);
vk_default!(VkDeviceMemory_T);
vk_default!(VkEvent_T);
vk_default!(VkQueryPool_T);
vk_default!(VkBufferView_T);
vk_default!(VkImageView_T);
vk_default!(VkShaderModule_T);
vk_default!(VkPipelineCache_T);
vk_default!(VkPipelineLayout_T);
vk_default!(VkPipeline_T);
vk_default!(VkRenderPass_T);
vk_default!(VkDescriptorSetLayout_T);
vk_default!(VkSampler_T);
vk_default!(VkDescriptorSet_T);
vk_default!(VkDescriptorPool_T);
vk_default!(VkFramebuffer_T);
vk_default!(VkCommandPool_T);

pub fn vk_assert(result:VkResult) {
    assert!(result==0);
}

pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

pub struct Context {
    pub instance : VkInstance,
    pub physical_devices : Vec<VkPhysicalDevice>,
}

impl Context {
    pub fn new() -> Self {

        let mut instance = vk_instantiate!(VkInstance);
        let mut physical_devices = vec![];

        // instance
        unsafe {
            let app_name = CString::new("vkcholesky").unwrap();
            let ref_app_name = &app_name;
            let eng_name = CString::new("No engine").unwrap();
            let ref_eng_name = &eng_name;

            let app_info = VkApplicationInfo {
                sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: null(),
                pApplicationName: ref_app_name.as_ptr(),
                applicationVersion: make_version(1, 0, 0),
                pEngineName: ref_eng_name.as_ptr(),
                engineVersion: make_version(1, 0, 0),
                apiVersion: make_version(1, 0, 0),
            };              
            
            // careful to CString lifetime
            let layers = CString::new("VK_LAYER_KHRONOS_validation").unwrap();
            let ref_layers = &layers;
            let pp_layers = vec![ref_layers.as_ptr()];
            let extensions = CString::new("VK_EXT_debug_report").unwrap();
            let ref_extensions = &extensions;
            let pp_extensions = vec![ref_extensions.as_ptr()];

            let mut count:u32 = 10;
            let mut layer_prop = VkLayerProperties {
                layerName: [0;256],
                specVersion: 0,
                implementationVersion: 0,
                description: [0;256],
            };

            let mut instance_layer_prop = vec![layer_prop;count as usize];
            vkEnumerateInstanceLayerProperties(&mut count, instance_layer_prop.as_mut_ptr());
            let layers : Vec<*const i8> = instance_layer_prop.iter().map(|x| x.layerName.as_ptr()).collect();

            println!("instance");

            let instance_create_info = VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: null(),
                flags: 0,
                pApplicationInfo:  &app_info,
                enabledLayerCount: 1,
                ppEnabledLayerNames: pp_layers.as_ptr(),
                enabledExtensionCount: 1,
                ppEnabledExtensionNames: pp_extensions.as_ptr(),
            };
            vk_assert(vkCreateInstance(&instance_create_info, null(), &mut instance));

            let mut count:u32 = 3;
            let mut layer_prop = VkLayerProperties {
                layerName: [0;256],
                specVersion: 0,
                implementationVersion: 0,
                description: [0;256],
            };

            let mut instance_layer_prop = vec![layer_prop;count as usize];
            vkEnumerateInstanceLayerProperties(&mut count, instance_layer_prop.as_mut_ptr());
        }
        // phyiscal device
        unsafe {
            let mut device_count = 0 as u32;
            vk_assert(vkEnumeratePhysicalDevices(instance, &mut device_count, null_mut()));
            physical_devices = vec![vk_instantiate!(VkPhysicalDevice); device_count as usize];
            vk_assert(vkEnumeratePhysicalDevices(instance, &mut device_count, physical_devices.as_mut_ptr()));
        }
        
        Self { instance:instance, physical_devices:physical_devices } 
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

fn vulkan_context() -> &'static Context {
    static mut CTX: MaybeUninit<Context> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CTX.as_mut_ptr().write(
            Context::new()
        );
    });

    unsafe { &*CTX.as_ptr() }
}

pub struct Device {
    pub self_ : VkDevice,
    pub queue : VkQueue,
    pub queue_family_index : u32,
    pub command_pool : VkCommandPool,
}

impl Device {
    pub fn new() -> Self {
        let mut device = vk_instantiate!(VkDevice);
        let mut queue = vk_instantiate!(VkQueue);
        let mut queue_family_index:u32 = 0;
        let mut command_pool = vk_instantiate!(VkCommandPool);

        let ctx = vulkan_context();

        // queue family index
        unsafe {

            let mut qf_count = 0;
            vkGetPhysicalDeviceQueueFamilyProperties(ctx.physical_devices[0], &mut qf_count, null_mut());

            // dummy
            let qf_prop_inst = VkQueueFamilyProperties { 
                queueFlags: 0, 
                queueCount: 0, 
                timestampValidBits: 0, 
                minImageTransferGranularity: VkExtent3D { width: 0, height: 0, depth: 0 }, 
            };
            let mut qf_props = vec![qf_prop_inst;qf_count as usize];
            vkGetPhysicalDeviceQueueFamilyProperties(ctx.physical_devices[0], &mut qf_count, qf_props.as_mut_ptr());

            let clt_cmpts:Vec<usize> = qf_props
                .iter()
                .enumerate()
                .filter(|(_i, x)| (x.queueFlags & (VK_QUEUE_COMPUTE_BIT as u32)) != 0)
                .map(|(i, _x)| i).collect();
            queue_family_index = clt_cmpts[0] as u32;
        }

        println!("device");
        // device
        unsafe {
            let queue_priority = 1.0;
            let dvc_q_crt_info = VkDeviceQueueCreateInfo {
                sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                pNext: null(),
                flags: 0,
                queueFamilyIndex: queue_family_index,
                queueCount: 1,
                pQueuePriorities: &queue_priority,
            };

            let dvc_crt_info = VkDeviceCreateInfo {
                sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
                pNext: null(),
                flags: 0,
                queueCreateInfoCount: 1,
                pQueueCreateInfos: &dvc_q_crt_info,
                enabledLayerCount: 0,
                ppEnabledLayerNames: null(),
                enabledExtensionCount: 0,
                ppEnabledExtensionNames: null(),
                pEnabledFeatures: null(),
            };
            vk_assert(vkCreateDevice(ctx.physical_devices[0], &dvc_crt_info, null(), &mut device));
            vkGetDeviceQueue(device, queue_family_index, 0, &mut queue);
        }

        println!("command Pool");
        // command pool
        unsafe {
            let cmd_pool_crt_info = VkCommandPoolCreateInfo {
                sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
                pNext: null(),
                flags: 0,
                queueFamilyIndex: queue_family_index,
            };
            vk_assert(vkCreateCommandPool(device, &cmd_pool_crt_info, null(), &mut command_pool));
        }

        Self { self_:device, queue:queue, queue_family_index:queue_family_index, command_pool:command_pool }
    }
}

pub struct Buffer<'a, T> {
    device : &'a VkDevice,
    self_ : VkBuffer,
    memory : VkDeviceMemory,
    data : Vec<T>,
}

impl<'a, T> Buffer<'a, T> {

    pub fn new(info:VkBufferCreateInfo, data:Vec<T>, device:&'a VkDevice) -> Self {
        
        let mut buf = vk_instantiate!(VkBuffer);
        let mem = vk_instantiate!(VkDeviceMemory);
        unsafe {
            vk_assert(vkCreateBuffer(*device, &info, null(), &mut buf));
        }

        Self { self_:buf, memory:mem, data:data, device:device }
    }

    pub fn alloc(&mut self, mem_prop_flags: VkMemoryPropertyFlags) {
        let ctx = vulkan_context();
        unsafe {
            // dummy
            let mut mem_prop = VkPhysicalDeviceMemoryProperties {
                memoryTypeCount: 0,
                memoryTypes: [VkMemoryType{ propertyFlags: 0, heapIndex: 0 };32],
                memoryHeapCount: 0,
                memoryHeaps: [VkMemoryHeap{ size: 0, flags: 0 };16],
            };
            vkGetPhysicalDeviceMemoryProperties(ctx.physical_devices[0], &mut mem_prop);

            let mut mem_req = VkMemoryRequirements {
                size: 0,
                alignment: 0,
                memoryTypeBits: 0,
            };
            vkGetBufferMemoryRequirements(*self.device, self.self_, &mut mem_req);

            let mut mem_alloc_info = VkMemoryAllocateInfo {
                sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: null(),
                allocationSize: mem_req.size,
                memoryTypeIndex: 0,
            };

            for i in 0..mem_prop.memoryTypeCount {
                if mem_req.memoryTypeBits&1 == 1 {
                    if mem_prop.memoryTypes[i as usize].propertyFlags&mem_prop_flags == mem_prop_flags {
                        mem_alloc_info.memoryTypeIndex = i;
                    }
                }
            }
            vk_assert(vkAllocateMemory(*self.device, &mut mem_alloc_info, null(), &mut self.memory));
        }
    }
}

pub struct Descriptor<'a> {
    device : &'a VkDevice,
    pool: VkDescriptorPool,
    pub sets: Vec<VkDescriptorSet>,
    pub set_layouts: Vec<VkDescriptorSetLayout>,
    pub count: u32,
}

impl<'a> Descriptor<'a> {

    pub fn new(count: u32, device:&'a VkDevice) -> Self {

        let mut pool = vk_instantiate!(VkDescriptorPool);
        let mut set_layout = vk_instantiate!(VkDescriptorSetLayout);
        let mut sets = vec![vk_instantiate!(VkDescriptorSet); count as usize];
        let mut set_layouts = vec![vk_instantiate!(VkDescriptorSetLayout); count as usize];

        println!("init");

        // descriptor pool
        unsafe {
            let desc_pool_size = VkDescriptorPoolSize {
                type_ : VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
                descriptorCount : count,
            };

            let desc_pool_create_info = VkDescriptorPoolCreateInfo {
                sType: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
                pNext: null(),
                flags: 0,
                maxSets: 1,
                poolSizeCount: 1,
                pPoolSizes: &desc_pool_size,
            }; 
            vk_assert(vkCreateDescriptorPool(*device, &desc_pool_create_info, null(), &mut pool));
        }
        println!("pool");

        // descriptor layout
        unsafe {
            let desc_set_layout_binding = VkDescriptorSetLayoutBinding {
                binding: 0,
                descriptorType: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
                descriptorCount: count,
                stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as u32,
                pImmutableSamplers: null(),
            };

            let desc_set_layout_create_info = VkDescriptorSetLayoutCreateInfo {
                sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
                pNext: null(),
                flags: 0,
                bindingCount: 1,
                pBindings: &desc_set_layout_binding,
            };
            vk_assert(vkCreateDescriptorSetLayout(*device, &desc_set_layout_create_info, null(), &mut set_layout));
        }

        // descriptor set
        unsafe {
            let desc_set_allocate_info = VkDescriptorSetAllocateInfo {
                sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
                pNext: null(),
                descriptorPool: pool,
                descriptorSetCount: count,
                pSetLayouts: &set_layout,
            };
            vk_assert(vkAllocateDescriptorSets(*device, &desc_set_allocate_info, sets.as_mut_ptr()));
        }

        Self { device:device, pool:pool, sets:sets, set_layouts:vec![set_layout], count:count }
    }

    pub fn update(&self, write_desc_set:Vec<VkWriteDescriptorSet>) {
        unsafe {
            vkUpdateDescriptorSets(
                *self.device, 
                write_desc_set.len() as u32, 
                write_desc_set.as_ptr(), 
                0, null());
        }
    }
}

pub struct CommandBuffer {
    self_: VkCommandBuffer,
}

pub struct ComputePipeline {
    self_: VkPipeline,
    layout : VkPipelineLayout,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t_context() {
        let ctx = Context::new();
    }

}