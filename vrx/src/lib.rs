#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use std::any::{type_name, Any};

include!("vkstruct.rs");
include!("vktraits.rs");

pub mod memory;

pub fn vk_assert(result: VkResult) {
    assert!(result == VkResult::VK_SUCCESS, "VkResult: {:?}", result);
}

pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

#[macro_use]
pub mod func_static {

use paste::paste;

#[macro_export]
macro_rules! load_spv {
    ( $x:tt ) => {{
        include_bytes!(x)
    }};
}

#[macro_export]
macro_rules! vk_instantiate {
    ( $x:ident ) => {{
        paste::paste! {
            let mut type_T = [<$x _T>]::default();
            let mut type_inst : *mut [<$x _T>] = &mut type_T;
        }

        type_inst
    }};
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

            (BIND_INDEX_BUFFER($buffer:expr, $size:expr, $index_type:expr)) => {
                vkCmdBindIndexBuffer(
                    $cmd,
                    $buffer,
                    $size,
                    $index_type,
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

            (DRAW_INDEXED(
                $index_count:expr,
                $instance_count:expr,
                $first_index:expr,
                $vertex_offset:expr,
                $first_instance:expr
            )) => {
                vkCmdDrawIndexed(
                    $cmd,
                    $index_count,
                    $instance_count,
                    $first_index,
                    $vertex_offset,
                    $first_instance,
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

#[macro_export]
macro_rules! vkMakeBind {
    () => {};
}

pub use vkCmdBlock;
pub use vkMakeBind;
    pub use vk_instantiate;
} // the end of module

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
            format: VkFormat::VK_FORMAT_UNDEFINED,
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

#[derive(Eq, Hash, PartialEq, Copy, Clone, PartialOrd, Ord, Debug)]
pub enum QueueType {
    graphics,
    computes,
    transfer,
    none,
}

#[derive(Debug)]
pub struct VulkanHandler {
    pub device: VkDevice,
    pub queues: HashMap<(u32, u32), VkQueue>,

    // all sorted by queue familly indices
    command_pools: Vec<VkCommandPool>, // command pools per queue family indices
    queue_types: Vec<QueueType>,       // Vec<u32> map to command pools
}

impl VulkanHandler {
    // pub fn new(demands: Vec<(QueueType, u32)>) -> Self {
    pub fn new(demands: &[(QueueType, &[f32])]) -> Self {
        let ctx = vulkan_context();

        let queue_type_map = |queue_type: QueueType| -> VkQueueFlagBits {
            match queue_type {
                QueueType::graphics => VK_QUEUE_GRAPHICS_BIT,
                QueueType::computes => VK_QUEUE_COMPUTE_BIT,
                QueueType::transfer => VK_QUEUE_TRANSFER_BIT,
                QueueType::none => 0,
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

        new_device
    }

    pub fn destroy(&mut self) {
        unsafe {
            // self.command_pools
            //     .iter()
            //     .for_each(|(t, commad_pool)| self.device.destroy_command_pool(*commad_pool, None));
            vkDestroyDevice(self.device, null());
        }
    }

    //
    pub fn get_command_pool(&self, queue_type: &QueueType, index: usize) -> VkCommandPool {
        self.command_pools[self.queue_family_indices.get(queue_type).unwrap()[index] as usize]
    }

    //
    // command buffer
    pub fn allocate_command_buffers(
        &self,
        queue_type: &QueueType,
        index: usize,
        level: VkCommandBufferLevel,
        count: u32,
    ) -> Vec<VkCommandBuffer> {
        let command_pool = self.get_command_pool(queue_type, index);

        let info = VkCommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .level(level)
            .command_buffer_count(count)
            .build();

        self.device.allocate_command_buffers(&info)
    }

    pub fn free_commands_buffers(
        &self,
        command_pool: VkCommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const VkCommandBuffer,
    ) {
        unsafe {
            vkFreeCommandBuffers(
                self.device,
                command_pool,
                command_buffer_count,
                p_command_buffers,
            )
        }
    }

    //
    // high level api
    //
    // descriptor
    pub fn create_descriptor(&self, descriptor_pool_sizes: &[VkDescriptorPoolSize]) -> anyhow::Result<Descriptor> {
        Ok(Descriptor::new(descriptor_pool_sizes, &self.device))
    }

    // buffer
    pub fn create_buffer<'a, T>(
        &'a self,
        data: (Option<*const T>, usize),
        usage: VkBufferUsageFlagBits,
        flags: VkBufferCreateFlagBits,
        mem_prop_flags: VkMemoryPropertyFlagBits,
    ) -> anyhow::Result<Buffer<T>> {
        Ok(Buffer::<T>::new(
            data,
            flags as VkBufferCreateFlags,
            usage as VkBufferUsageFlags,
            mem_prop_flags as VkMemoryPropertyFlags,
            &self.device,
        ))
    }

    pub fn create_staging_buffer(

    ) {

    }

    // Textures
    // raw texture
    // pub fn create_texture<'a>(
    //     &'a self,
    //     mem_prop_info: VkMemoryPropertyFlagBits,
    // ) -> anyhow::Result<Texture> {
    //     Ok(Texture::new(mem_prop_info, &self.device))
    // }

    // texture 2D
    // pub fn create_texture2D<'a>() -> Result<Texture> {

    // }
} 