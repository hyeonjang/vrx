#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use paste::paste;
use std::any::{type_name, Any};

include!("vk_traits.rs");

pub mod graphics;

#[macro_export]
macro_rules! load_spv {
    ( $x:tt ) => {{
        include_bytes!(x)
    }};
}

#[macro_export]
macro_rules! vk_instantiate {
    ( $x:ident ) => {{
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

            (BIND_VERTEX_BUFFERS($first_binding:expr, $binding_count:expr, $p_buffers:expr, $p_offsets:expr)) => {
                vkCmdBindVertexBuffers(
                    $cmd, 
                    $first_binding,
                    $binding_count,
                    $p_buffers,
                    $p_offsets,
                );
            };

            (COPY_BUFFER($source:expr, $target:expr, $num:expr, $buffer_copy:expr)) => {
                vkCmdCopyBuffer($cmd, $source, $target, $num, $buffer_copy);
            };

            (COPY_BUFFER_TO_IMAGE($buffer:expr, $image:expr, $image_layout:expr, $region_count:expr, $p_regions:expr)) => {
                vkCmdCopyBufferToImage($cmd, $buffer, $image, $image_layout, $region_count, $p_regions);
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

            (DRAW(
                $vertex_count:expr,
                $instance_count:expr,
                $first_vertex:expr,
                $first_instance:expr)) => {
                    vkCmdDraw(
                        $cmd,
                        $vertex_count,
                        $instance_count,
                        $first_vertex,
                        $first_instance
                    );
            };

            (BEGIN_RENDER_PASS(
                $render_pass_begin_info:expr,
                $vk_subpass_contents:expr
            )) => {
                vkCmdBeginRenderPass(
                    $cmd,
                    $render_pass_begin_info,
                    $vk_subpass_contents
                );
            };

            (END_RENDER_PASS()) => {
                vkCmdEndRenderPass(
                    $cmd
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

const EXTENTION: &[u8] = b"VK_EXT_debug_report\nVK_KHR_surface\nVK_KHR_win32_surface";

#[derive(Debug)]
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
            let lunarg_layers = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
            let ref_layers = &layers;
            let pp_layers = vec![ref_layers.as_ptr(), lunarg_layers.as_ptr()];
            //@@todo automatical extension
            let extensions = CString::new("VK_EXT_debug_report").unwrap();
            let surf_extensions = CString::new("VK_KHR_surface").unwrap();
            let win32_extensions = CString::new("VK_KHR_win32_surface").unwrap();
            let ref_extensions = &extensions;
            let pp_extensions = vec![
                ref_extensions.as_ptr(),
                surf_extensions.as_ptr(),
                win32_extensions.as_ptr(),
            ];

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

            let instance_create_info = VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: null(),
                flags: 0,
                pApplicationInfo: &app_info,
                enabledLayerCount: 1,
                ppEnabledLayerNames: pp_layers.as_ptr(),
                enabledExtensionCount: pp_extensions.len() as u32,
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

    pub fn get_physical_device_surface_support_khr(
        &self,
        queue_family_index: u32,
        surface: VkSurfaceKHR,
    ) -> VkBool32 {
        let mut supported = VK_FALSE;
        unsafe {
            vk_assert(vkGetPhysicalDeviceSurfaceSupportKHR(
                self.physical_devices[0],
                queue_family_index,
                surface,
                &mut supported,
            ))
        }
        supported
    }

    pub fn get_physical_device_surface_capabilities_khr(
        &self,
        surface: &VkSurfaceKHR,
    ) -> VkSurfaceCapabilitiesKHR {
        let null_extent = VkExtent2D {
            width: 0,
            height: 0,
        };

        let mut surface_capabilities_khr = VkSurfaceCapabilitiesKHR {
            minImageCount: 0,
            maxImageCount: 0,
            currentExtent: null_extent,
            minImageExtent: null_extent,
            maxImageExtent: null_extent,
            maxImageArrayLayers: 0,
            supportedTransforms: 0,
            currentTransform: 0,
            supportedCompositeAlpha: 0,
            supportedUsageFlags: 0,
        };

        unsafe {
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                self.physical_devices[0],
                *surface,
                &mut surface_capabilities_khr,
            );
        }

        surface_capabilities_khr
    }

    pub fn get_physical_device_surface_formats_khr(
        &self,
        surface: &VkSurfaceKHR,
    ) -> Vec<VkSurfaceFormatKHR> {
        let mut count = 0;
        unsafe {
            vkGetPhysicalDeviceSurfaceFormatsKHR(
                self.physical_devices[0],
                *surface,
                &mut count,
                null_mut(),
            );
        }

        let surface_format_dummy = VkSurfaceFormatKHR {
            format: 0,
            colorSpace: 0,
        };
        let mut surface_formats_khr: Vec<VkSurfaceFormatKHR> =
            vec![surface_format_dummy.clone(); count as usize];
        unsafe {
            vkGetPhysicalDeviceSurfaceFormatsKHR(
                self.physical_devices[0],
                *surface,
                &mut count,
                surface_formats_khr.as_mut_ptr(),
            );
        }

        surface_formats_khr
    }

    pub fn get_physical_device_surface_present_modes_khr(
        &self,
        surface: &VkSurfaceKHR,
    ) -> Vec<VkPresentModeKHR> {
        let mut count = 0;
        unsafe {
            vkGetPhysicalDeviceSurfacePresentModesKHR(
                self.physical_devices[0],
                *surface,
                &mut count,
                null_mut(),
            );
        }

        let mut surface_present_modes_khr: Vec<VkPresentModeKHR> = vec![0; count as usize];
        unsafe {
            vkGetPhysicalDeviceSurfacePresentModesKHR(
                self.physical_devices[0],
                *surface,
                &mut count,
                surface_present_modes_khr.as_mut_ptr(),
            );
        }
        surface_present_modes_khr
    }

    fn get_physical_device_queue_familly_properties(&self) -> Vec<VkQueueFamilyProperties> {
        let mut queue_family_count = 0;

        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                self.physical_devices[0],
                &mut queue_family_count,
                null_mut(),
            );
        }

        let queue_family_properties = VkQueueFamilyProperties {
            queueFlags: 0,
            queueCount: 0,
            timestampValidBits: 0,
            minImageTransferGranularity: VkExtent3D {
                width: 0,
                height: 0,
                depth: 0,
            },
        };
        let mut queue_familly_properties =
            vec![queue_family_properties; queue_family_count as usize];
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                self.physical_devices[0],
                &mut queue_family_count,
                queue_familly_properties.as_mut_ptr(),
            );
        }
        queue_familly_properties
    }

    #[cfg(all(target_os = "windows", feature = "graphics"))]
    pub fn create_win32_surface_khr(
        &self,
        win32_surface_create_info: *const VkWin32SurfaceCreateInfoKHR,
        p_allocator: Option<*const VkAllocationCallbacks>,
    ) -> VkSurfaceKHR {
        let mut surface = vk_instantiate!(VkSurfaceKHR);

        unsafe {
            if let Some(p) = p_allocator {
                vkCreateWin32SurfaceKHR(
                    self.instance,
                    win32_surface_create_info,
                    p_allocator.unwrap(),
                    &mut surface,
                );
            } else {
                vkCreateWin32SurfaceKHR(
                    self.instance,
                    win32_surface_create_info,
                    null(),
                    &mut surface,
                );
            }
        }
        surface
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self.instance, null());
        }
    }
}

// singleton
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
#[derive(Eq, Hash, PartialEq, Copy, Clone, PartialOrd, Ord, Debug)]
pub enum QueueType {
    graphics,
    computes,
    transfer,
}

// same as vulkanalia
#[derive(Clone, Debug)]
pub struct SwapchainSupport {
    pub capabilities: VkSurfaceCapabilitiesKHR,
    pub formats: Vec<VkSurfaceFormatKHR>,
    pub present_modes: Vec<VkPresentModeKHR>,
}

impl SwapchainSupport {
    pub fn new(surface: &VkSurfaceKHR) -> Self {
        let ctx = vulkan_context();
        Self {
            capabilities: ctx.get_physical_device_surface_capabilities_khr(&surface),
            formats: ctx.get_physical_device_surface_formats_khr(&surface),
            present_modes: ctx.get_physical_device_surface_present_modes_khr(&surface),
        }
    }

    pub fn get_swapchain_surface_format(
        &self,
        format: VkFormat,
        color_space: VkColorSpaceKHR,
    ) -> VkSurfaceFormatKHR {
        self.formats
            .iter()
            .cloned()
            .find(|f| f.format == format && f.colorSpace == color_space)
            .unwrap_or_else(|| self.formats[0])
    }

    pub fn get_swapchain_present_mode(&self, present_mode: VkPresentModeKHR) -> VkPresentModeKHR {
        self.present_modes
            .iter()
            .cloned()
            .find(|m| *m == present_mode)
            .unwrap_or(VK_PRESENT_MODE_FIFO_KHR)
    }

    pub fn get_swapchain_extent(&self) -> VkExtent2D {
        self.capabilities.currentExtent
    }
}

#[derive(Debug)]
pub struct VulkanResourceHandler {
    pub device: VkDevice,
    pub queue_family_indices: HashMap<QueueType, Vec<u32>>, // Vec<u32> map to command pools
    pub command_pools: Vec<VkCommandPool>, // command pools per queue family indices
                                           // pub descriptor_pools
}

impl VulkanResourceHandler {
    // pub fn new(demands: Vec<(QueueType, u32)>) -> Self {
    pub fn new(demands: &[(QueueType, &[f32])]) -> Self {
        let ctx = vulkan_context();

        let queue_type_map = |queue_type: QueueType| -> VkQueueFlagBits {
            match queue_type {
                QueueType::graphics => VK_QUEUE_GRAPHICS_BIT,
                QueueType::computes => VK_QUEUE_COMPUTE_BIT,
                QueueType::transfer => VK_QUEUE_TRANSFER_BIT,
            }
        };

        // queue family index
        demands
            .to_vec()
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut queue_family_indices: HashMap<QueueType, Vec<u32>> = HashMap::new();
        let mut device_queue_create_infos: Vec<VkDeviceQueueCreateInfo> = Vec::new();

        // 1.
        let find_queue_family = |flag: VkQueueFlagBits| -> Vec<u32> {
            let index_collected: Vec<u32> = ctx
                .get_physical_device_queue_familly_properties()
                .iter()
                .enumerate()
                .filter(|(_i, x)| (x.queueFlags & (flag as u32)) != 0)
                .map(|(i, _x)| i as u32)
                .collect();
            index_collected
        };

        // 2. initialize queue family index
        demands.iter().for_each(|demand| {
            queue_family_indices.insert(demand.0, vec![]);
        });

        // 3. resource (indices) allocation
        let mut tot_indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
        let mut process_queue_family = |queue_type: QueueType, queue_priorities: &[f32]| {
                let q_inds = find_queue_family(queue_type_map(queue_type));
                let mut index = 0;
                for i in q_inds {
                    for (j, ind) in tot_indices.iter().enumerate() {
                        if i == *ind {
                            index = tot_indices.remove(j);
                            break;
                        }
                    }
                }
                let mut queue_type_indices = queue_family_indices.get_mut(&queue_type).unwrap();
                queue_type_indices.push(index);

                let device_queue_create_info = VkDeviceQueueCreateInfoBuilder::new()
                .queue_family_index(index as u32)
                .queue_count(queue_priorities.len() as u32)
                .p_queue_priorities(queue_priorities.as_ptr())
                    .build();
                device_queue_create_infos.push(device_queue_create_info);
            };

        // 3. real execute
        demands
            .iter()
            .for_each(|demand| process_queue_family(demand.0, demand.1));

        // device
        let vk_khr_swapchain = b"VK_KHR_swapchain\0".as_ptr() as *const i8;
        let extensions = [vk_khr_swapchain];
        let mut device = vk_instantiate!(VkDevice);
        let device_create_info = VkDeviceCreateInfoBuilder::new()
            .queue_create_info_count(device_queue_create_infos.len() as u32)
            .p_queue_create_infos(device_queue_create_infos.as_ptr())
            .enabled_extension_count(extensions.len() as u32)
            .pp_enabled_extension_names(extensions.as_ptr())
            .build();

        device = ctx.physical_devices[0].create_device(&device_create_info, None);

        let mut command_pools: Vec<VkCommandPool> = vec![vk_instantiate!(VkCommandPool); 5];
        for (queue_family, indices) in &queue_family_indices {
            indices.iter().for_each(|i| {
                let command_pool_create_info = VkCommandPoolCreateInfoBuilder::new()
                    .queue_family_index(*i)
                    .build();
                command_pools[*i as usize] =
                    device.create_command_pool(&command_pool_create_info, None);
            })
        }

        // command pool
        let mut new_device = Self {
            device: device,
            queue_family_indices: queue_family_indices,
            command_pools: command_pools,
        };
        // new_device.new_command_pool();

        new_device
    }

    // fn new_command_pool(&mut self) {
    //     let command_pools = self
    //         .queue_family_indices
    //         .iter()
    //         .map(|(queue_family_index, indices)| {
    //             let command_pool_create_info = VkCommandPoolCreateInfoBuilder::new()
    //                 .queue_family_index(indices[0])
    //                 .build();
    //             let command_pool = self
    //                 .device
    //                 .create_command_pool(&command_pool_create_info, None);
    //             // .unwrap();
    //             // self.command_pools.insert(*queue_type, command_pool);
    //             (indices[0], command_pool)
    //         })
    //         .collect::<Vec<(u32, VkCommandPool)>>();

    //     // command_pools.iter().for_each(move |(u32, pool)| {
    //     //     self.command_pools.insert(*queue_type, *pool);
    //     // });
    // }

    pub fn destroy(&mut self) {
        unsafe {
            // self.command_pools
            //     .iter()
            //     .for_each(|(t, commad_pool)| self.device.destroy_command_pool(*commad_pool, None));
            vkDestroyDevice(self.device, null());
        }
    }

    pub fn allocate_descriptor_sets(
        &self,
        descriptor_set_alloc_info: &VkDescriptorSetAllocateInfo,
    ) -> Result<Vec<VkDescriptorSet>> {
        let mut descriptor_sets = vec![
            vk_instantiate!(VkDescriptorSet);
            descriptor_set_alloc_info.descriptorSetCount as usize
        ];

        unsafe {
            vkAllocateDescriptorSets(
                self.device,
                descriptor_set_alloc_info,
                descriptor_sets.as_mut_ptr(),
            );
        }
        Ok(descriptor_sets)
    }

    pub fn update_descriptor_sets(
        &self,
        descriptor_write_count: usize,
        p_descriptor_writes: *const VkWriteDescriptorSet,
    ) {
        unsafe {
            vkUpdateDescriptorSets(
                self.device,
                descriptor_write_count as u32,
                p_descriptor_writes,
                0,
                null(),
            );
        }
    }

    //
    // descriptor
    pub fn create_descriptor(&self, set_count: u32) -> Result<Descriptor> {
        Ok(Descriptor::new(set_count, &self.device))
    }

    //
    // command buffer
    pub fn allocate_command_buffer(
        &self,
        command_type: QueueType,
        level: VkCommandBufferLevel,
    ) -> Result<VkCommandBuffer> {
        let mut cmd_buf = vk_instantiate!(VkCommandBuffer);

        let info = VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: null(),
            commandPool: *(self.command_pools.get(&command_type)).unwrap(),
            level: level,
            commandBufferCount: 1,
        };
        unsafe {
            vk_assert(vkAllocateCommandBuffers(self.device, &info, &mut cmd_buf));
        }
        Ok(cmd_buf)
    }

    pub fn allocate_command_buffers(
        &self,
        command_type: QueueType,
        level: VkCommandBufferLevel,
        count: u32,
    ) -> Result<Vec<VkCommandBuffer>> {
        let mut cmd_bufs = vec![vk_instantiate!(VkCommandBuffer); count as usize];

        let info = VkCommandBufferAllocateInfoBuilder::new()
            .command_pool(*self.command_pools.get(&command_type).unwrap())
            .level(level)
            .command_buffer_count(count)
            .build();

        unsafe {
            vk_assert(vkAllocateCommandBuffers(
                self.device,
                &info,
                cmd_bufs.as_mut_ptr(),
            ));
        }
        Ok(cmd_bufs)
    }

    pub fn free_commands_buffers(
        &self,
        command_type: QueueType,
        command_buffer_count: u32,
        p_command_buffers: *const VkCommandBuffer,
    ) {
        unsafe {
            vkFreeCommandBuffers(
                self.device,
                *self.command_pools.get(&command_type).unwrap(),
                command_buffer_count,
                p_command_buffers,
            )
        }
    }

    //
    // high level api
    //
    pub fn create_vxbuffer<'a, T>(
        &'a self,
        data: *const T,
        len: u32,
        usage: VkBufferUsageFlagBits,
        flags: VkBufferCreateFlags,
        mem_prop_flags: VkMemoryPropertyFlagBits,
    ) -> Result<VxBuffer<T>> {
        Ok(VxBuffer::<T>::new(
            data,
            len,
            flags,
            usage as VkBufferUsageFlags,
            mem_prop_flags,
            &self.device,
        ))
    }

    pub fn create_texture<'a>(
        &'a self,
        image_create_info: VkImageCreateInfo,
        mem_prop_info: VkMemoryPropertyFlagBits,
    ) -> Result<Texture> {
        Ok(Texture::new(image_create_info, mem_prop_info, &self.device))
    }
}

pub trait Memory {
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
    fn bind_buffer_memory(&self, offset: VkDeviceSize) {}
    fn bind_image_memory(&self, offset: VkDeviceSize) {}
}

#[derive(Debug)]
pub struct VxBuffer<'a, T> {
    device: &'a VkDevice,

    pub buffer: VkBuffer,
    memory: VkDeviceMemory,

    data: *const T,
    len: u32,
}

impl<'a, T> Memory for VxBuffer<'a, T> {
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
    fn bind_buffer_memory(&self, offset: VkDeviceSize) {
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

impl<'a, T> VxBuffer<'a, T> {
    pub fn new(
        data: *const T,
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
            copy_nonoverlapping(buffer.data, mapped.cast(), buffer.len as usize);
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
            copy_nonoverlapping(self.data, mapped.cast(), self.len as usize);
        }
        self.unmap_memory();
    }

    pub fn map_to_cpu_and_unmap(&mut self) -> Vec<T>
    where
        T: std::clone::Clone + Default,
    {
        let mut output = vec![T::default(); self.len as usize];

        let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
        unsafe {
            copy_nonoverlapping(mapped.cast(), output.as_mut_ptr(), self.len as usize);
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
}

pub struct Texture<'a> {
    device: &'a VkDevice,

    image: VkImage,
    memory: VkDeviceMemory,

    info: VkImageCreateInfo,
}

impl<'a> Memory for Texture<'a> {
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
            vkGetImageMemoryRequirements(*self.device(), *self.image(), &mut mem_req);

            mem_req
        }
    }

    fn bind_buffer_memory(&self, offset: VkDeviceSize) {
        unimplemented!();
    }
    fn bind_image_memory(&self, offset: VkDeviceSize) {
        unsafe {
            vk_assert(vkBindImageMemory(
                *self.device,
                self.image,
                self.memory,
                offset,
            ));
        }
    }
}

impl<'a> Texture<'a> {
    pub fn new(
        info: VkImageCreateInfo,
        mem_prop_flags: VkMemoryPropertyFlagBits,
        device: &'a VkDevice,
    ) -> Self {
        let mut image = vk_instantiate!(VkImage);
        unsafe {
            vk_assert(vkCreateImage(*device, &info, null(), &mut image));
        }

        let memory = vk_instantiate!(VkDeviceMemory);
        let mut vximage = Self {
            device: device,
            image: image,
            memory: memory,
            info: info,
        };

        vximage.allocate_memory(mem_prop_flags);
        vximage.bind_image_memory(0);

        vximage
    }

    pub fn image(&self) -> &VkImage {
        &self.image
    }

    pub fn create_image_view(&self) -> VkImageView {
        let mut image_view = vk_instantiate!(VkImageView);
        let image_view_create_info = VkImageViewCreateInfo {
            sType: todo!(),
            pNext: todo!(),
            flags: todo!(),
            image: self.image,
            viewType: todo!(),
            format: self.info.format,
            components: todo!(),
            subresourceRange: todo!(),
        };

        unsafe {
            vkCreateImageView(
                *self.device,
                &image_view_create_info,
                null(),
                &mut image_view,
            );
        }

        image_view
    }

    pub fn get_image_sub_resource_layers(&self) {}
}

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

// @@todo redesign
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
