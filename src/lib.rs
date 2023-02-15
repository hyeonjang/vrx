#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("vulkan_header.rs");

use std::any::{type_name, Any};
use std::ffi::*;
use std::mem::*;
use std::ptr::*;
use std::ptr::{copy_nonoverlapping as memcpy, null};
use std::str::*;
use std::sync::{Mutex, Once};

use anyhow::*;
use paste::paste;

use phf::phf_map;

pub static STRUCTURE_TYPE_CREATE_INFO_MAP: phf::Map<&str, VkStructureType> = phf_map! {
    "VkBufferCreateInfo" => VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
    "VkImageCreateInfo" => VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
    "VkDescriptorPoolCreateInfo" => VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
    "VkDescriptorSetLayoutCreateInfo" => VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
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
    ( pub struct $type_:ty { $(pub $field:ident: $field_type:ty $(,)?)* } ) => {

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
    pub struct VkImageCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkImageCreateFlags,
        pub imageType: VkImageType,
        pub format: VkFormat,
        pub extent: VkExtent3D,
        pub mipLevels: u32,
        pub arrayLayers: u32,
        pub samples: VkSampleCountFlagBits,
        pub tiling: VkImageTiling,
        pub usage: VkImageUsageFlags,
        pub sharingMode: VkSharingMode,
        pub queueFamilyIndexCount: u32,
        pub pQueueFamilyIndices: *const u32,
        pub initialLayout: VkImageLayout,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkBufferCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkBufferCreateFlags,
        pub size: VkDeviceSize,
        pub usage: VkBufferUsageFlags,
        pub sharingMode: VkSharingMode,
        pub queueFamilyIndexCount: u32,
        pub pQueueFamilyIndices: *const u32,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkDescriptorPoolCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkDescriptorPoolCreateFlags,
        pub maxSets: u32,
        pub poolSizeCount: u32,
        pub pPoolSizes: *const VkDescriptorPoolSize,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkDescriptorSetLayoutCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkDescriptorSetLayoutCreateFlags,
        pub bindingCount: u32,
        pub pBindings: *const VkDescriptorSetLayoutBinding,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkWriteDescriptorSet {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub dstSet: VkDescriptorSet,
        pub dstBinding: u32,
        pub dstArrayElement: u32,
        pub descriptorCount: u32,
        pub descriptorType: VkDescriptorType,
        pub pImageInfo: *const VkDescriptorImageInfo,
        pub pBufferInfo: *const VkDescriptorBufferInfo,
        pub pTexelBufferView: *const VkBufferView,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkCommandBufferBeginInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkCommandBufferUsageFlags,
        pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkSubmitInfo {
        pub sType: VkStructureType,
        pub waitSemaphoreCount: u32,
        pub pWaitSemaphores: *const VkSemaphore,
        pub pWaitDstStageMask: *const VkPipelineStageFlags,
        pub commandBufferCount: u32,
        pub pCommandBuffers: *const VkCommandBuffer,
        pub signalSemaphoreCount: u32,
        pub pSignalSemaphores: *const VkSemaphore,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkFenceCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkFenceCreateFlags,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineCacheCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineCacheCreateFlags,
        pub initialDataSize: usize,
        pub pInitialData: *const ::std::os::raw::c_void,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineShaderStageCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineShaderStageCreateFlags,
        pub stage: VkShaderStageFlagBits,
        pub module: VkShaderModule,
        pub pName: *const ::std::os::raw::c_char,
        pub pSpecializationInfo: *const VkSpecializationInfo,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineLayoutCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineLayoutCreateFlags,
        pub setLayoutCount: u32,
        pub pSetLayouts: *const VkDescriptorSetLayout,
        pub pushConstantRangeCount: u32,
        pub pPushConstantRanges: *const VkPushConstantRange,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkComputePipelineCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineCreateFlags,
        pub stage: VkPipelineShaderStageCreateInfo,
        pub layout: VkPipelineLayout,
        pub basePipelineHandle: VkPipeline,
        pub basePipelineIndex: i32,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkMappedMemoryRange {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub memory: VkDeviceMemory,
        pub offset: VkDeviceSize,
        pub size: VkDeviceSize,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkBufferMemoryBarrier {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub srcAccessMask: VkAccessFlags,
        pub dstAccessMask: VkAccessFlags,
        pub srcQueueFamilyIndex: u32,
        pub dstQueueFamilyIndex: u32,
        pub buffer: VkBuffer,
        pub offset: VkDeviceSize,
        pub size: VkDeviceSize,
    }
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

            (PUSH_CONSTANT(
                $layout: expr,
                $stageFlags: expr,
                $offset: expr,
                $size: expr,
                $pValues: expr
            )) => {
                vkCmdPushConstants(
                    $cmd, $layout, $stageFlags, $offset, $size, $pValues
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

//
// higher-level wrapper
//
pub mod vx {

    use crate::*;

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

        pub fn get_phyiscal_device_properties(&self) -> VkPhysicalDeviceProperties {
            let mut physical_device_properties = VkPhysicalDeviceProperties::default();

            unsafe {
                vkGetPhysicalDeviceProperties(
                    self.physical_devices[0],
                    &mut physical_device_properties,
                );
            }

            physical_device_properties
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

    pub fn vulkan_context() -> &'static Context {
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

    // impl<'a,  T> VkWrapper<T> for VxBuffer<'a, T> {
    //     type VkStruct = VkBuffer;

    //     fn as_raw(&self) -> Self::VkStruct {
    //         self.buffer
    //     }

    //     fn as_raw_ptr(&self) -> &Self::VkStruct {
    //         &self.buffer
    //     }
    // }

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
        pub fn create_buffer(
            &self,
            create_info: *const VkBufferCreateInfo,
            p_allocator: Option<*const VkAllocationCallbacks>,
        ) -> Result<VkBuffer> {
            let mut buffer = vk_instantiate!(VkBuffer);
            unsafe {
                if let Some(p) = p_allocator {
                    vkCreateBuffer(self.self_, create_info, p, &mut buffer);
                } else {
                    vkCreateBuffer(self.self_, create_info, null(), &mut buffer);
                }
            }
            Ok(buffer)
        }

        //
        // descriptor
        pub fn create_descriptor(&self, set_count: u32) -> Result<Descriptor> {
            Ok(Descriptor::new(set_count, &self.self_))
        }

        //
        // command buffer
        pub fn allocate_command_buffer(
            &self,
            level: VkCommandBufferLevel,
        ) -> Result<VkCommandBuffer> {
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

        pub fn destroy_fence(
            &self,
            fence: VkFence,
            p_allocator: Option<*const VkAllocationCallbacks>,
        ) {
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

        //
        // high level api
        //
        pub fn create_vxbuffer<'a, 'b, T>(
            &'b self,
            data: &'a mut T,
            len: u32,
            usage: VkBufferUsageFlagBits,
            flags: VkBufferCreateFlags,
            mem_prop_flags: VkMemoryPropertyFlagBits,
        ) -> Result<VxBuffer<T>>
        where
            'a: 'b,
        {
            Ok(VxBuffer::<T>::new(
                data,
                len,
                flags,
                usage as VkBufferUsageFlags,
                mem_prop_flags,
                &self.self_,
            ))
        }
    }

    trait Memory {
        // trait getter
        fn device(&self) -> &VkDevice;
        fn buffer(&self) -> Option<&VkBuffer> {
            None
        }
        fn buffer_mut(&mut self) -> Option<&mut VkBuffer> {
            None
        }
        fn image(&self) -> Option<&VkImage> {
            None
        }
        fn image_mut(&mut self) -> Option<&mut VkImage> {
            None
        }
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
                let mut mapped = MaybeUninit::<*mut c_void>::uninit();

                vk_assert(vkMapMemory(
                    *self.device(),
                    *self.memory(),
                    offset,
                    size,
                    flags,
                    mapped.as_mut_ptr(),
                ));

                Ok(mapped.assume_init())
            }
        }

        fn unmap_memory(&self) {
            unsafe {
                vkUnmapMemory(*self.device(), *self.memory());
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

        fn flush_mapped_memory_range(
            &self,
            memory_range_count: u32,
            p_memory_ranges: *const VkMappedMemoryRange,
        ) {
            unsafe {
                vkFlushMappedMemoryRanges(*self.device(), memory_range_count, p_memory_ranges);
            }
        }

        fn invalidate_mapped_memory_ranges(
            &self,
            memory_range_count: u32,
            p_memory_ranges: *const VkMappedMemoryRange,
        ) {
            unsafe {
                vkInvalidateMappedMemoryRanges(*self.device(), memory_range_count, p_memory_ranges);
            }
        }
    }

    #[derive(Debug)]
    pub struct VxBuffer<'a, 'b, T> {
        device: &'a VkDevice,

        buffer: VkBuffer,
        memory: VkDeviceMemory,

        data: &'b mut T,
        len: u32,
    }

    impl<'a, 'b, T> Memory for VxBuffer<'a, 'b, T> {
        fn device(&self) -> &VkDevice {
            self.device
        }

        fn buffer(&self) -> Option<&VkBuffer> {
            Some(&self.buffer)
        }

        fn buffer_mut(&mut self) -> Option<&mut VkBuffer> {
            Some(&mut self.buffer)
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

    impl<'a, 'b, T> VxBuffer<'a, 'b, T> {
        pub fn new(
            data: &'b mut T,
            len: u32,
            flags: VkBufferCreateFlags,
            usage: VkBufferUsageFlags,
            mem_prop_flags: VkMemoryPropertyFlagBits,
            device: &'a VkDevice,
        ) -> Self {
            let mut buf = vk_instantiate!(VkBuffer);
            let mem = vk_instantiate!(VkDeviceMemory);

            let info = VkBufferCreateInfo {
                sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
                pNext: null(),
                flags: flags,
                size: (len * size_of::<T>() as u32) as u64,
                usage: usage,
                sharingMode: VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,    // no working here
                pQueueFamilyIndices: null(), // no working here
            };

            unsafe {
                vk_assert(vkCreateBuffer(*device, &info, null(), &mut buf));
            }

            let mut buffer = Self {
                buffer: buf,
                memory: mem,
                data: data,
                len: len,
                device: device,
            };

            buffer.allocate_memory(mem_prop_flags);
            buffer.bind_buffer_memory(0);

            let mapped = buffer.map_memory(0, buffer.vksize(), 0).unwrap();
            unsafe {
                memcpy(buffer.data, mapped.cast(), buffer.len as usize);
            }
            buffer.unmap_memory();

            buffer
        }

        pub fn buffer(&self) -> &VkBuffer {
            &self.buffer
        }

        pub fn map_to_gpu_and_unmap(&self) {
            let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
            unsafe {
                memcpy(self.data, mapped.cast(), self.len as usize);
            }
            self.unmap_memory();
        }

        pub fn map_to_cpu_and_unmap(&mut self) -> Vec<T> where T: std::clone::Clone + Default {

            let mut output = vec![T::default(); self.len as usize];

            let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
            unsafe {
                memcpy(mapped.cast(), output.as_mut_ptr(), self.len as usize);
            }
            self.unmap_memory();

            output
        }

        pub fn vksize(&self) -> VkDeviceSize {
            (self.len * size_of::<T>() as u32) as VkDeviceSize
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

    pub struct VxImage<'a, T> {
        device: &'a VkDevice,

        image: VkImage,
        memory: VkDeviceMemory,

        pub data: Vec<T>,
    }

    impl<'a, T> Memory for VxImage<'a, T> {
        fn device(&self) -> &VkDevice {
            self.device
        }

        fn image(&self) -> Option<&VkImage> {
            Some(&self.image)
        }

        fn image_mut(&mut self) -> Option<&mut VkImage> {
            Some(&mut self.image)
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
                vkGetImageMemoryRequirements(*self.device(), *self.image().unwrap(), &mut mem_req);

                mem_req
            }
        }
    }

    // impl<'a, T> VxImage<'a, T> {
    //     pub fn new(
    //         device: &'a VkDevice,
    //         flags: VkBufferCreateFlags,
    //         usage: VkBufferUsageFlags,
    //         data: Vec<T>,
    //     ) -> Self {
    //         let mut buf = vk_instantiate!(VkImage);
    //         unsafe {
    //             let info = VkBufferCreateInfo {
    //                 sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
    //                 pNext: null(),
    //                 flags: flags,
    //                 size: (size_of::<T>() * data.len()) as u64,
    //                 usage: usage,
    //                 sharingMode: VK_SHARING_MODE_EXCLUSIVE,
    //                 queueFamilyIndexCount: 0,    // no working here
    //                 pQueueFamilyIndices: null(), // no working here
    //             };
    //             vk_assert(vkCreateBuffer(*device, &info, null(), &mut buf));
    //         }

    //         let mem = vk_instantiate!(VkDeviceMemory);
    //         Self {
    //             buffer: buf,
    //             memory: mem,
    //             data: data,
    //             device: device,
    //         }
    //     }
    // }

    pub struct PushConstant<T> {
        stage: VkShaderStageFlags,
        data: *const T,
        size: u32,
    }

    impl<T> PushConstant<T> {
        pub fn new(stage: VkShaderStageFlagBits, data: *const T, size: u32) -> Self {
            Self {
                stage: stage as u32,
                data: data,
                size: size,
            }
        }

        pub fn vksize(&self) -> u32 {
            (self.size * size_of::<T>() as u32) as u32
        }

        pub fn as_ptr(&self) -> *const std::os::raw::c_void {
            self.data as *const std::os::raw::c_void
        }

        pub fn stage(&self) -> VkShaderStageFlags {
            self.stage
        }

        pub fn range(&self) -> VkPushConstantRange {
            VkPushConstantRange {
                stageFlags: self.stage,
                offset: 0,
                size: self.vksize(),
            }
        }

        pub fn range_custom(&self, offset: u32, size: u32) -> VkPushConstantRange {
            VkPushConstantRange {
                stageFlags: self.stage,
                offset: offset,
                size: size,
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
}
