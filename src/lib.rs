#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("bindings.rs");

use std::any::{type_name, Any};
use std::ffi::*;
use std::mem::*;
use std::ptr::*;
use std::str::*;
use std::sync::{Mutex, Once};

use anyhow::*;
use paste::paste;

use phf::phf_map;

pub static STRUCTURE_TYPE_CREATE_INFO_MAP: phf::Map<&str, VkStructureType> = phf_map! {
    "VkWriteDescriptorSet" => VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
    "VkCommandBufferBeginInfo" => VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
    "VkSubmitInfo" => VK_STRUCTURE_TYPE_SUBMIT_INFO,
    "VkFenceCreateInfo" => VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
    "VkMappedMemoryRange" => VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
    "VkBufferMemoryBarrier" => VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
    "VkPipelineCacheCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
    "VkPipelineLayoutCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    "VkPipelineShaderStageCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    "VkComputePipelineCreateInfo" => VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
};

macro_rules! impl_builder_for_vk_structure_t {
    ( $type_:ty, $($field:ident: $field_type:ty $(,)?)* ) => {

        paste! {
            // builder contain info structure
            pub struct [<$type_ Builder>] {
                info: $type_,
            }

            impl [<$type_ Builder>] {
                pub fn new() -> [<$type_ Builder>] {
                    let mut builder = [<$type_ Builder>] {
                        info:$type_::default()
                    };

                    let type_string = stringify!($type_);
                    let sType = STRUCTURE_TYPE_CREATE_INFO_MAP.get(type_string);
                    if let Some(x) = sType {
                        builder.info.sType = *sType.unwrap();
                    } else {
                        panic!("No mapped structure type for {}, please insert", type_string);
                    }

                    builder
                }

                pub fn build(self) -> $type_ {
                    // ?error checking is possible

                    self.info
                }

                $(
                    pub fn $field(mut self, $field:$field_type) -> [<$type_ Builder>] {
                        self.info.$field = $field;
                        self
                    }
                )*
            }
        }
    };
}

// InfoBuilder implementations
impl_builder_for_vk_structure_t!(
    VkWriteDescriptorSet,
    pNext: *const ::std::os::raw::c_void,
    dstSet: VkDescriptorSet,
    dstBinding: u32,
    dstArrayElement: u32,
    descriptorCount: u32,
    descriptorType: VkDescriptorType,
    pImageInfo: *const VkDescriptorImageInfo,
    pBufferInfo: *const VkDescriptorBufferInfo,
    pTexelBufferView: *const VkBufferView,
);

impl_builder_for_vk_structure_t!(
    VkCommandBufferBeginInfo,
    pNext: *const ::std::os::raw::c_void,
    flags: VkCommandBufferUsageFlags,
    pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
);

impl_builder_for_vk_structure_t!(
    VkSubmitInfo,
    waitSemaphoreCount: u32,
    pWaitSemaphores: *const VkSemaphore,
    pWaitDstStageMask: *const VkPipelineStageFlags,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
    signalSemaphoreCount: u32,
    pSignalSemaphores: *const VkSemaphore,
);

impl_builder_for_vk_structure_t!(
    VkFenceCreateInfo,
    pNext: *const ::std::os::raw::c_void,
    flags: VkFenceCreateFlags,
);

impl_builder_for_vk_structure_t!(
    VkPipelineCacheCreateInfo,
    pNext: *const ::std::os::raw::c_void,
    flags: VkPipelineCacheCreateFlags,
    initialDataSize: usize,
    pInitialData: *const ::std::os::raw::c_void,
);

impl_builder_for_vk_structure_t!(
    VkPipelineShaderStageCreateInfo,
    pNext: *const ::std::os::raw::c_void,
    flags: VkPipelineShaderStageCreateFlags,
    stage: VkShaderStageFlagBits,
    module: VkShaderModule,
    pName: *const ::std::os::raw::c_char,
    pSpecializationInfo: *const VkSpecializationInfo,
);

impl_builder_for_vk_structure_t!(
    VkPipelineLayoutCreateInfo,
    pNext: *const ::std::os::raw::c_void,
    flags: VkPipelineLayoutCreateFlags,
    setLayoutCount: u32,
    pSetLayouts: *const VkDescriptorSetLayout,
    pushConstantRangeCount: u32,
    pPushConstantRanges: *const VkPushConstantRange,
);

impl_builder_for_vk_structure_t!(
    VkComputePipelineCreateInfo,
    pNext: *const ::std::os::raw::c_void,
    flags: VkPipelineCreateFlags,
    stage: VkPipelineShaderStageCreateInfo,
    layout: VkPipelineLayout,
    basePipelineHandle: VkPipeline,
    basePipelineIndex: i32,
);

impl_builder_for_vk_structure_t!(
    VkMappedMemoryRange,
    pNext: *const ::std::os::raw::c_void,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
);

impl_builder_for_vk_structure_t!(
    VkBufferMemoryBarrier,
    pNext: *const ::std::os::raw::c_void,
    srcAccessMask: VkAccessFlags,
    dstAccessMask: VkAccessFlags,
    srcQueueFamilyIndex: u32,
    dstQueueFamilyIndex: u32,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize,
);

macro_rules! impl_default_for_vk_pointer_t {
    ( $x:ident ) => {
        impl Default for $x {
            fn default() -> $x {
                $x { _unused: [0; 0] }
            }
        }
    };
}

impl_default_for_vk_pointer_t!(VkBuffer_T);
impl_default_for_vk_pointer_t!(VkImage_T);
impl_default_for_vk_pointer_t!(VkInstance_T);
impl_default_for_vk_pointer_t!(VkPhysicalDevice_T);
impl_default_for_vk_pointer_t!(VkDevice_T);
impl_default_for_vk_pointer_t!(VkQueue_T);
impl_default_for_vk_pointer_t!(VkSemaphore_T);
impl_default_for_vk_pointer_t!(VkCommandBuffer_T);
impl_default_for_vk_pointer_t!(VkFence_T);
impl_default_for_vk_pointer_t!(VkDeviceMemory_T);
impl_default_for_vk_pointer_t!(VkEvent_T);
impl_default_for_vk_pointer_t!(VkQueryPool_T);
impl_default_for_vk_pointer_t!(VkBufferView_T);
impl_default_for_vk_pointer_t!(VkImageView_T);
impl_default_for_vk_pointer_t!(VkShaderModule_T);
impl_default_for_vk_pointer_t!(VkPipelineCache_T);
impl_default_for_vk_pointer_t!(VkPipelineLayout_T);
impl_default_for_vk_pointer_t!(VkPipeline_T);
impl_default_for_vk_pointer_t!(VkRenderPass_T);
impl_default_for_vk_pointer_t!(VkDescriptorSetLayout_T);
impl_default_for_vk_pointer_t!(VkSampler_T);
impl_default_for_vk_pointer_t!(VkDescriptorSet_T);
impl_default_for_vk_pointer_t!(VkDescriptorPool_T);
impl_default_for_vk_pointer_t!(VkFramebuffer_T);
impl_default_for_vk_pointer_t!(VkCommandPool_T);

#[macro_export]
macro_rules! load_spv {
    ( $x:tt ) => {{
        include_bytes!(x)
    }};
}

#[macro_export]
macro_rules! vk_instantiate {
    ( $x:ident ) => {{
        use paste::paste;

        paste! {
            let mut type_T = [<$x _T>]::default();
            let mut type_inst : *mut [<$x _T>] = &mut type_T;
        }

        type_inst
    }};
}

pub fn vk_assert(result: VkResult) {
    assert!(result == VkResult::VK_SUCCESS, "VkResult: {:?}", result);
}

pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

//
// higher-level wrapper
//
pub struct Context {
    pub instance: VkInstance,
    pub physical_devices: Vec<VkPhysicalDevice>,
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

            let mut count: u32 = 10;
            let mut layer_prop = VkLayerProperties {
                layerName: [0; 256],
                specVersion: 0,
                implementationVersion: 0,
                description: [0; 256],
            };

            let mut instance_layer_prop = vec![layer_prop; count as usize];
            vkEnumerateInstanceLayerProperties(&mut count, instance_layer_prop.as_mut_ptr());
            let layers: Vec<*const i8> = instance_layer_prop
                .iter()
                .map(|x| x.layerName.as_ptr())
                .collect();

            println!("instance");

            let instance_create_info = VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: null(),
                flags: 0,
                pApplicationInfo: &app_info,
                enabledLayerCount: 1,
                ppEnabledLayerNames: pp_layers.as_ptr(),
                enabledExtensionCount: 1,
                ppEnabledExtensionNames: pp_extensions.as_ptr(),
            };
            vk_assert(vkCreateInstance(
                &instance_create_info,
                null(),
                &mut instance,
            ));

            let mut count: u32 = 3;
            let mut layer_prop = VkLayerProperties {
                layerName: [0; 256],
                specVersion: 0,
                implementationVersion: 0,
                description: [0; 256],
            };

            let mut instance_layer_prop = vec![layer_prop; count as usize];
            vkEnumerateInstanceLayerProperties(&mut count, instance_layer_prop.as_mut_ptr());
        }
        // phyiscal device
        unsafe {
            let mut device_count = 0 as u32;
            vk_assert(vkEnumeratePhysicalDevices(
                instance,
                &mut device_count,
                null_mut(),
            ));
            physical_devices = vec![vk_instantiate!(VkPhysicalDevice); device_count as usize];
            vk_assert(vkEnumeratePhysicalDevices(
                instance,
                &mut device_count,
                physical_devices.as_mut_ptr(),
            ));
        }

        Self {
            instance: instance,
            physical_devices: physical_devices,
        }
    }

    pub fn get_physical_device_memory_properties(&self) -> VkPhysicalDeviceMemoryProperties {
        let mut mem_prop = VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: 0,
            memoryTypes: [VkMemoryType {
                propertyFlags: 0,
                heapIndex: 0,
            }; 32],
            memoryHeapCount: 0,
            memoryHeaps: [VkMemoryHeap { size: 0, flags: 0 }; 16],
        };

        unsafe {
            vkGetPhysicalDeviceMemoryProperties(self.physical_devices[0], &mut mem_prop);
        }
        mem_prop
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
        CTX.as_mut_ptr().write(Context::new());
    });

    unsafe { &*CTX.as_ptr() }
}

///
/// High-level wrapping trait for Custom VkStructure
///
pub trait VkWrapper<T> {
    type VkStruct;

    fn as_raw(&self) -> Self::VkStruct;
    fn as_raw_ptr(&self) -> &Self::VkStruct;
}

impl<T> VkWrapper<T> for Device {
    type VkStruct = VkDevice;

    fn as_raw(&self) -> Self::VkStruct {
        self.self_
    }

    fn as_raw_ptr(&self) -> &Self::VkStruct {
        &self.self_
    }
}

impl<'a, T> VkWrapper<T> for Buffer<'a, T> {
    type VkStruct = VkBuffer;

    fn as_raw(&self) -> Self::VkStruct {
        self.buffer
    }

    fn as_raw_ptr(&self) -> &Self::VkStruct {
        &self.buffer
    }
}

pub struct Device {
    pub self_: VkDevice,
    pub queue_family_index: u32,
    pub command_pool: VkCommandPool,
}

impl Device {
    pub fn new() -> Self {
        let mut device = vk_instantiate!(VkDevice);
        let mut queue_family_index: u32 = 0;
        let mut command_pool = vk_instantiate!(VkCommandPool);

        let ctx = vulkan_context();

        // queue family index
        unsafe {
            let mut qf_count = 0;
            vkGetPhysicalDeviceQueueFamilyProperties(
                ctx.physical_devices[0],
                &mut qf_count,
                null_mut(),
            );

            // dummy
            let qf_prop_inst = VkQueueFamilyProperties {
                queueFlags: 0,
                queueCount: 0,
                timestampValidBits: 0,
                minImageTransferGranularity: VkExtent3D {
                    width: 0,
                    height: 0,
                    depth: 0,
                },
            };
            let mut qf_props = vec![qf_prop_inst; qf_count as usize];
            vkGetPhysicalDeviceQueueFamilyProperties(
                ctx.physical_devices[0],
                &mut qf_count,
                qf_props.as_mut_ptr(),
            );

            let clt_cmpts: Vec<usize> = qf_props
                .iter()
                .enumerate()
                .filter(|(_i, x)| (x.queueFlags & (VK_QUEUE_COMPUTE_BIT as u32)) != 0)
                .map(|(i, _x)| i)
                .collect();
            queue_family_index = clt_cmpts[0] as u32;
        }

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
            vk_assert(vkCreateDevice(
                ctx.physical_devices[0],
                &dvc_crt_info,
                null(),
                &mut device,
            ));
        }

        // command pool
        unsafe {
            let cmd_pool_crt_info = VkCommandPoolCreateInfo {
                sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
                pNext: null(),
                flags: 0,
                queueFamilyIndex: queue_family_index,
            };
            vk_assert(vkCreateCommandPool(
                device,
                &cmd_pool_crt_info,
                null(),
                &mut command_pool,
            ));
        }

        Self {
            self_: device,
            queue_family_index: queue_family_index,
            command_pool: command_pool,
        }
    }

    //
    // vkqueues
    pub fn get_queue(&self, index: usize) -> Result<VkQueue> {
        let mut queue = vk_instantiate!(VkQueue);
        unsafe {
            vkGetDeviceQueue(
                self.self_,
                self.queue_family_index,
                index as u32,
                &mut queue,
            );
        };
        Ok(queue)
    }

    pub fn queue_submit(
        &self,
        index: usize,
        infos: *const VkSubmitInfo,
        info_count: u32,
        fence: VkFence,
    ) {
        let queue = self.get_queue(index).unwrap();

        unsafe {
            vk_assert(vkQueueSubmit(queue, info_count, infos, fence));
        }
    }

    pub fn create_shader_module(&self, code: &[u8]) -> Result<VkShaderModule> {
        let mut module = vk_instantiate!(VkShaderModule);

        unsafe {
            let code = Vec::<u8>::from(code);
            let (prefix, code_u32, suffix) = code.align_to::<u32>();
            if !prefix.is_empty() || !suffix.is_empty() {
                return Err(anyhow!("Load code for module failed"));
            }

            let shader_create_info = VkShaderModuleCreateInfo {
                sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
                pNext: null(),
                flags: 0,
                codeSize: code.len(),
                pCode: code_u32.as_ptr(),
            };

            vk_assert(vkCreateShaderModule(
                self.self_,
                &shader_create_info,
                null(),
                &mut module,
            ));
        }
        Ok(module)
    }

    //
    // shared vkbuffer
    pub fn create_buffer<T>(
        &self,
        data: Vec<T>,
        usage: VkBufferUsageFlagBits,
        flags: VkBufferCreateFlags,
    ) -> Result<Buffer<T>> {
        let info = VkBufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: null(),
            flags: flags,
            size: (size_of::<T>() * data.len()) as u64,
            usage: usage as VkBufferUsageFlags,
            sharingMode: VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: self.queue_family_index, // no working here
            pQueueFamilyIndices: &self.queue_family_index,  // no working here
        };

        Ok(Buffer::<T>::new(
            data,
            flags,
            usage as VkBufferUsageFlags,
            &self.self_,
        ))
    }

    //
    // descriptor
    pub fn create_descriptor(&self, set_count: u32) -> Result<Descriptor> {
        Ok(Descriptor::new(set_count, &self.self_))
    }

    //
    // command buffer
    pub fn allocate_command_buffer(&self, level: VkCommandBufferLevel) -> Result<VkCommandBuffer> {
        let mut cmd_buf = vk_instantiate!(VkCommandBuffer);

        let info = VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: null(),
            commandPool: self.command_pool,
            level: level,
            commandBufferCount: 1,
        };
        unsafe {
            vk_assert(vkAllocateCommandBuffers(self.self_, &info, &mut cmd_buf));
        }
        Ok(cmd_buf)
    }

    pub fn allocate_command_buffers(
        &self,
        level: VkCommandBufferLevel,
        count: u32,
    ) -> Result<Vec<VkCommandBuffer>> {
        let mut cmd_bufs = vec![vk_instantiate!(VkCommandBuffer); count as usize];

        let info = VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: null(),
            commandPool: self.command_pool,
            level: level,
            commandBufferCount: count,
        };

        unsafe {
            vk_assert(vkAllocateCommandBuffers(
                self.self_,
                &info,
                cmd_bufs.as_mut_ptr(),
            ));
        }
        Ok(cmd_bufs)
    }

    //
    // fence
    pub fn create_fence(
        &self,
        info: VkFenceCreateInfo,
        p_allocator: Option<*const VkAllocationCallbacks>,
    ) -> Result<VkFence> {
        let mut fence = vk_instantiate!(VkFence);

        unsafe {
            if let Some(p) = p_allocator {
                vkCreateFence(self.self_, &info, p_allocator.unwrap(), &mut fence);
            } else {
                vkCreateFence(self.self_, &info, null(), &mut fence);
            }
        }
        Ok(fence)
    }

    pub fn wait_for_fence(
        &self,
        fence_count: u32,
        p_fence: *const VkFence,
        wait_all: bool,
        timeout: u64,
    ) {
        unsafe {
            vk_assert(vkWaitForFences(
                self.self_,
                fence_count,
                p_fence,
                wait_all as VkBool32,
                timeout,
            ));
        }
    }

    pub fn reset_fence(&self, fence_count: u32, p_fences: *const VkFence) {
        unsafe {
            vkResetFences(self.self_, fence_count, p_fences);
        }
    }

    pub fn destroy_fence(&self, fence: VkFence, p_allocator: Option<*const VkAllocationCallbacks>) {
        unsafe {
            if let Some(p) = p_allocator {
                vkDestroyFence(self.self_, fence, p);
            } else {
                vkDestroyFence(self.self_, fence, null());
            }
        }
    }

    pub fn free_commands_buffers(
        &self,
        command_buffer_count: u32,
        p_command_buffers: *const VkCommandBuffer,
    ) {
        unsafe {
            vkFreeCommandBuffers(
                self.self_,
                self.command_pool,
                command_buffer_count,
                p_command_buffers,
            )
        }
    }

    pub fn create_pipeline_cache(
        &self,
        pipeline_cache_create_info: *const VkPipelineCacheCreateInfo,
    ) -> Result<VkPipelineCache> {
        let mut pipeline_cache = vk_instantiate!(VkPipelineCache);

        unsafe {
            vk_assert(vkCreatePipelineCache(
                self.self_,
                pipeline_cache_create_info,
                null(),
                &mut pipeline_cache,
            ));
        }
        Ok(pipeline_cache)
    }

    pub fn create_pipeline_layout(
        &self,
        pipeline_layout_create_info: *const VkPipelineLayoutCreateInfo,
    ) -> Result<VkPipelineLayout> {
        let mut pipeline_layout = vk_instantiate!(VkPipelineLayout);

        unsafe {
            vk_assert(vkCreatePipelineLayout(
                self.self_,
                pipeline_layout_create_info,
                null(),
                &mut pipeline_layout,
            ));
        }
        Ok(pipeline_layout)
    }

    pub fn create_compute_pipelines(
        &self,
        pipeline_cache: VkPipelineCache,
        create_info_count: u32,
        pipeline_create_infos: *const VkComputePipelineCreateInfo,
    ) -> Result<Vec<VkPipeline>> {
        let mut pipelines = vec![vk_instantiate!(VkPipeline); create_info_count as usize];

        unsafe {
            vk_assert(vkCreateComputePipelines(
                self.self_,
                pipeline_cache,
                create_info_count,
                pipeline_create_infos,
                null(),
                pipelines.as_mut_ptr(),
            ));
        }

        Ok(pipelines)
    }
}

pub trait Memory {
    // trait getter
    fn device(&self) -> &VkDevice;
    fn memory(&self) -> &VkDeviceMemory;
    fn memory_mut(&mut self) -> &mut VkDeviceMemory;

    /// functional
    fn get_memory_requirements(&self) -> VkMemoryRequirements;
    fn allocate_memory(&mut self, mem_prop_flags: VkMemoryPropertyFlagBits) {
        let ctx = vulkan_context();

        let mut mem_prop = ctx.get_physical_device_memory_properties();
        let mut collect: Vec<u32> = (0..mem_prop.memoryTypeCount).collect();
        collect.retain(|i| {
            mem_prop.memoryTypes[*i as usize].propertyFlags
                & mem_prop_flags as VkMemoryPropertyFlags
                == mem_prop_flags as VkMemoryPropertyFlags
        });

        let mut mem_req = self.get_memory_requirements();
        let mut mem_alloc_info = VkMemoryAllocateInfo {
            sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: null(),
            allocationSize: mem_req.size,
            memoryTypeIndex: collect[0],
        };

        unsafe {
            vk_assert(vkAllocateMemory(
                *self.device(),
                &mut mem_alloc_info,
                null(),
                self.memory_mut(),
            ));
        }
    }

    fn map_memory(&self, offset: u64, size: u64, flags: u32) -> Result<*mut c_void> {
        unsafe {
            // let mut mapped: *mut c_void = null_mut();
            // let mut array: Vec<T> = Vec::<T>::with_capacity(self.data.len());
            // array.set_len(self.data.len());
            let mut mapped = MaybeUninit::<*mut c_void>::uninit();

            vk_assert(vkMapMemory(
                *self.device(),
                *self.memory(),
                offset,
                size,
                flags,
                mapped.as_mut_ptr(),
            ));

            // println!("{:?}", array_ptr_c_void);
            // println!("{:?}", array_ptr_c_void.clone());
            // println!("{:?}", array_ptr_ptr);
            // array.copy_from_slice(self.data.as_slice());

            Ok(mapped.assume_init())
        }
    }

    fn free_memory(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        unsafe {
            if let Some(p) = p_allocator {
                vkFreeMemory(*self.device(), *self.memory(), p);
            } else {
                vkFreeMemory(*self.device(), *self.memory(), null());
            }
        }
    }
}

pub struct Buffer<'a, T> {
    device: &'a VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    pub data: Vec<T>,
}

impl<'a, T> Memory for Buffer<'a, T> {
    fn device(&self) -> &VkDevice {
        self.device
    }

    fn memory(&self) -> &VkDeviceMemory {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut VkDeviceMemory {
        &mut self.memory
    }

    fn get_memory_requirements(&self) -> VkMemoryRequirements {
        let ctx = vulkan_context();
        unsafe {
            let mut mem_req = VkMemoryRequirements {
                size: 0,
                alignment: 0,
                memoryTypeBits: 0,
            };
            vkGetBufferMemoryRequirements(*self.device(), self.buffer, &mut mem_req);

            mem_req
        }
    }
}

impl<'a, T> Buffer<'a, T> {
    pub fn new(
        data: Vec<T>,
        flags: VkBufferCreateFlags,
        usage: VkBufferUsageFlags,
        device: &'a VkDevice,
    ) -> Self {
        let mut buf = vk_instantiate!(VkBuffer);
        unsafe {
            let info = VkBufferCreateInfo {
                sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
                pNext: null(),
                flags: flags,
                size: (size_of::<T>() * data.len()) as u64,
                usage: usage,
                sharingMode: VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,    // no working here
                pQueueFamilyIndices: null(), // no working here
            };
            vk_assert(vkCreateBuffer(*device, &info, null(), &mut buf));
        }

        let mem = vk_instantiate!(VkDeviceMemory);
        Self {
            buffer: buf,
            memory: mem,
            data: data,
            device: device,
        }
    }

    pub fn vksize(&self) -> VkDeviceSize {
        (self.data.len() * size_of::<T>()) as VkDeviceSize
    }

    pub fn destroy(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        unsafe {
            if let Some(p) = p_allocator {
                vkDestroyBuffer(*self.device, self.buffer, p);
            } else {
                vkDestroyBuffer(*self.device, self.buffer, null());
            }
        }
    }

    pub fn unmap_memory(&self) {
        unsafe {
            vkUnmapMemory(*self.device, self.memory);
        }
    }

    pub fn flush_mapped_memory_range(
        &self,
        memory_range_count: u32,
        p_memory_ranges: *const VkMappedMemoryRange,
    ) {
        unsafe {
            vkFlushMappedMemoryRanges(*self.device, memory_range_count, p_memory_ranges);
        }
    }

    pub fn invalidate_mapped_memory_ranges(
        &self,
        memory_range_count: u32,
        p_memory_ranges: *const VkMappedMemoryRange,
    ) {
        unsafe {
            vkInvalidateMappedMemoryRanges(*self.device, memory_range_count, p_memory_ranges);
        }
    }

    pub fn bind_buffer_memory(&self, offset: VkDeviceSize) {
        unsafe {
            vk_assert(vkBindBufferMemory(
                *self.device,
                self.buffer,
                self.memory,
                offset,
            ));
        }
    }
}

pub struct Descriptor<'a> {
    device: &'a VkDevice,
    pool: VkDescriptorPool,
    pub sets: Vec<VkDescriptorSet>,
    pub set_layouts: Vec<VkDescriptorSetLayout>,
    pub count: u32,
}

impl<'a> Descriptor<'a> {
    pub fn new(count: u32, device: &'a VkDevice) -> Self {
        let mut pool = vk_instantiate!(VkDescriptorPool);
        let mut set_layout = vk_instantiate!(VkDescriptorSetLayout);
        let mut sets = vec![vk_instantiate!(VkDescriptorSet); count as usize];
        let mut set_layouts = vec![vk_instantiate!(VkDescriptorSetLayout); count as usize];

        // descriptor pool
        unsafe {
            let desc_pool_size = VkDescriptorPoolSize {
                type_: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
                descriptorCount: count,
            };

            let desc_pool_create_info = VkDescriptorPoolCreateInfo {
                sType: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
                pNext: null(),
                flags: 0,
                maxSets: 1,
                poolSizeCount: 1,
                pPoolSizes: &desc_pool_size,
            };
            vk_assert(vkCreateDescriptorPool(
                *device,
                &desc_pool_create_info,
                null(),
                &mut pool,
            ));
        }

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
            vk_assert(vkCreateDescriptorSetLayout(
                *device,
                &desc_set_layout_create_info,
                null(),
                &mut set_layout,
            ));
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
            vk_assert(vkAllocateDescriptorSets(
                *device,
                &desc_set_allocate_info,
                sets.as_mut_ptr(),
            ));
        }

        Self {
            device: device,
            pool: pool,
            sets: sets,
            set_layouts: vec![set_layout],
            count: count,
        }
    }

    pub fn update(&self, write_desc_set: Vec<VkWriteDescriptorSet>) {
        unsafe {
            vkUpdateDescriptorSets(
                *self.device,
                write_desc_set.len() as u32,
                write_desc_set.as_ptr(),
                0,
                null(),
            );
        }
    }
}

///
/// vulkan command block roles
///
#[macro_export]
macro_rules! vkCmdBlock {

    //
    // Parse the Vulkan Commands: the top (starting point) of parser
    //
    // * 'THIS' - indicator and identificator for starting cmd block
    // * 'cmd' - command buffer instance
    // * 'function' - command function for command buffer
    (THIS $cmd:expr; $function:ident($($args:expr),*);  $($tail:tt)*) => {

        let begin_info = VkCommandBufferBeginInfoBuilder::new()
        .flags(0)
        .build();

        unsafe {
            vk_assert(vkBeginCommandBuffer($cmd, &begin_info));

            vkCmdBlock!(@inner $cmd, $function($($args),*););
            vkCmdBlock!(@tt_recursion $cmd, $($tail)*);

            vk_assert(vkEndCommandBuffer($cmd));
        }
    };

    // * 'cmd' - command buffer instance
    // * 'let lv0 = rv0' - pre declarations
    (THIS $cmd:expr; let $lv:ident = $rv:expr; $($tail:tt)* ) => {

        let begin_info = VkCommandBufferBeginInfoBuilder::new()
            .flags(0)
            .build();

        unsafe {
            vk_assert(vkBeginCommandBuffer($cmd, &begin_info));

            let $lv= $rv;
            vkCmdBlock!(@tt_recursion $cmd, $($tail)*);

            vk_assert(vkEndCommandBuffer($cmd));
        }
    };

    //
    // tt recursive parser for the function call
    //
    (@tt_recursion $cmd:expr, $function:ident($($args:expr),*); $($tail:tt)*) => {
        vkCmdBlock!(@inner $cmd, $function($($args),*););
        vkCmdBlock!(@tt_recursion $cmd, $($tail)*);
    };

    // declaration
    (@tt_recursion $cmd:expr, let $lv0:ident = $rv0:expr; $($tail:tt)*) => {
        let $lv0 = $rv0;
        vkCmdBlock!(@tt_recursion $cmd, $($tail)*);
    };

    // empty
    (@tt_recursion $cmd:expr,) => {};

    //
    // Parse the Vulkan All Commands
    //
    // * '@inner' - identifier for inner macro
    // * 'cmd' - command buffer instance
    // * 'function' - command function for command buffer
    (@inner $cmd:expr, $function:ident($($args:expr),*);) => {

        macro_rules! inner {
            (BIND_DESCRIPTOR_SETS(
                $pipeline_bind_point:expr,
                $layout:expr,
                $first_set:expr,
                $descriptor_set_count:expr,
                $p_descriptor_sets:expr,
                $dynamic_offset_count:expr,
                $p_dynamic_offsets:expr
            )) => {
                vkCmdBindDescriptorSets(
                    $cmd,
                    $pipeline_bind_point,
                    $layout,
                    $first_set,
                    $descriptor_set_count,
                    $p_descriptor_sets,
                    $dynamic_offset_count,
                    $p_dynamic_offsets
                );
            };

            (BIND_PIPELINE($pipeline_bind_point:expr, $pipeline:expr)) => {
                vkCmdBindPipeline(
                    $cmd,
                    $pipeline_bind_point,
                    $pipeline,
                );
            };

            (COPY_BUFFER($source:expr, $target:expr, $num:expr, $buffer_copy:expr)) => {
                vkCmdCopyBuffer($cmd, $source, $target, $num, $buffer_copy);
            };

            (DISPATCH(
                $group_count_x:expr,
                $group_count_y:expr,
                $group_count_z:expr)) => {
                    vkCmdDispatch(
                        $cmd,
                        $group_count_x, $group_count_y, $group_count_z
                    );
            };

            (PIPELINE_BARRIER(
                $src_stage_mask:expr,
                $dst_stage_mask:expr,
                $dependency_flags:expr,
                $memory_barrier_count:expr,
                $p_memory_barriers:expr,
                $buffer_memory_barrier_count:expr,
                $p_buffer_memory_barriers:expr,
                $image_memory_barrier_count:expr,
                $p_image_memory_barriers:expr)) => {

                vkCmdPipelineBarrier(
                    $cmd,
                    $src_stage_mask.try_into().unwrap(),
                    $dst_stage_mask.try_into().unwrap(),
                    $dependency_flags,
                    $memory_barrier_count,
                    $p_memory_barriers,
                    $buffer_memory_barrier_count,
                    $p_buffer_memory_barriers,
                    $image_memory_barrier_count,
                    $p_image_memory_barriers
                );
            };


        } // the end of macro_rules! "inner"
        inner!($function($($args),*));
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t_context() {
        let ctx = Context::new();
    }
}
