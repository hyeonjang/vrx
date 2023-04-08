// #[cfg_attr(feature = "graphics",)]
include!("vk_graphics_header.rs");
// #[cfg_attr(feature = "computes",)]
// include!("vk_computes_header.rs");

use std::collections::HashMap;
use std::ffi::*;
use std::mem::*;
use std::ptr::*;
use std::ptr::{copy_nonoverlapping, null};
use std::str::*;
use std::sync::{Mutex, Once};

use anyhow::*;

use phf::phf_map;

pub const VK_COLOR_COMPONENT_ALL_BIT: VkColorComponentFlagBits = (VK_COLOR_COMPONENT_R_BIT
    | VK_COLOR_COMPONENT_B_BIT
    | VK_COLOR_COMPONENT_G_BIT
    | VK_COLOR_COMPONENT_A_BIT);

pub static STRUCTURE_TYPE_CREATE_INFO_MAP: phf::Map<&str, VkStructureType> = phf_map! {
    "VkDeviceQueueCreateInfo" => VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    "VkDeviceCreateInfo" => VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    "VkSwapchainCreateInfoKHR" => VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
    "VkCommandPoolCreateInfo" => VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    "VkBufferCreateInfo" => VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
    "VkImageCreateInfo" => VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
    "VkImageViewCreateInfo" => VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
    "VkSamplerCreateInfo" => VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
    "VkDescriptorPoolCreateInfo" => VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
    "VkDescriptorSetAllocateInfo" => VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
    "VkDescriptorSetLayoutCreateInfo" => VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
    "VkWriteDescriptorSet" => VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
    "VkCommandBufferBeginInfo" => VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
    "VkCommandBufferAllocateInfo" => VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    "VkSubmitInfo" => VK_STRUCTURE_TYPE_SUBMIT_INFO,
    "VkFenceCreateInfo" => VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
    "VkSemaphoreCreateInfo" => VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
    "VkMappedMemoryRange" => VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
    "VkBufferMemoryBarrier" => VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
    "VkPipelineVertexInputStateCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    "VkPipelineInputAssemblyStateCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    "VkPipelineViewportStateCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
    "VkPipelineRasterizationStateCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    "VkPipelineMultisampleStateCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    "VkPipelineColorBlendStateCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
    "VkPipelineDynamicStateCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    "VkRenderPassCreateInfo" => VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
    "VkRenderPassBeginInfo" => VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
    "VkFramebufferCreateInfo" => VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
    "VkPresentInfoKHR" => VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
    "VkPipelineCacheCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
    "VkPipelineLayoutCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    "VkPipelineShaderStageCreateInfo" => VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    "VkGraphicsPipelineCreateInfo" => VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
    "VkComputePipelineCreateInfo" => VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
    "VkFrameBufferCreateInfo" => VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
    "VkWin32SurfaceCreateInfoKHR" => VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
};

macro_rules! impl_create_function {
    ($subject:expr, $name:expr $(, $khr:expr)?) => {
        paste! {
            pub fn [<create_ $name:snake>](
                &self,
                [<$name:snake _create_info>]: *const [<Vk $name CreateInfo $($khr)?>],
                p_allocator: Option<*const VkAllocationCallbacks>,
            ) -> Result<[<Vk $name $($khr)?>]> {
                let mut instance = vk_instantiate!([<Vk $name $($khr)?>]);

                unsafe {
                    if let Some(p) = p_allocator {
                        [<vkCreate $name $($khr)?>](self.$subject, [<$name:snake _create_info>], p_allocator.unwrap(), &mut instance);
                    } else {
                        [<vkCreate $name $($khr)?>](self.$subject, [<$name:snake _create_info>], null(), &mut instance);
                    }
                }
                Ok(instance)
            }
        }
    };
}

macro_rules! impl_destroy_function {
    ($subject:expr, $name:expr $(, $khr:expr)?) => {
        paste! {
            pub fn [<destroy_ $name:snake>](
                &self,
                [<$name:snake>]: [<Vk $name $($khr)?>],
                p_allocator: Option<*const VkAllocationCallbacks>,
            ) {

                unsafe {
                    if let Some(p) = p_allocator {
                        [<vkDestroy $name $($khr)?>](self.$subject, [<$name:snake>], p_allocator.unwrap());
                    } else {
                        [<vkDestroy $name $($khr)?>](self.$subject, [<$name:snake>], null());
                    }
                }
            }
        }
    };
}

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
                    pub fn [<$field:snake>](mut self, [<$field:snake>]:$field_type) -> [<$type_ Builder>] {
                        self.info.$field = [<$field:snake>];
                        self
                    }
                )*
            }
        }
    };
}

macro_rules! impl_builder_for_vk_none_structure_t {
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

                    // let type_string = stringify!($type_);
                    // let sType = STRUCTURE_TYPE_CREATE_INFO_MAP.get(type_string);
                    // if let Some(x) = sType {
                    //     builder.info.sType = *sType.unwrap();
                    // } else {
                    //     panic!("No mapped structure type for {}, please insert", type_string);
                    // }

                    builder
                }

                pub fn build(self) -> $type_ {
                    // ?error checking is possible

                    self.info
                }

                $(
                    pub fn [<$field:snake>](mut self, [<$field:snake>]:$field_type) -> [<$type_ Builder>] {
                        self.info.$field = [<$field:snake>];
                        self
                    }
                )*
            }
        }
    };
}

// InfoBuilder implementations
impl_builder_for_vk_structure_t!(
    pub struct VkDeviceQueueCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkDeviceQueueCreateFlags,
        pub queueFamilyIndex: u32,
        pub queueCount: u32,
        pub pQueuePriorities: *const f32,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkDeviceCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkDeviceCreateFlags,
        pub queueCreateInfoCount: u32,
        pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
        pub enabledLayerCount: u32,
        pub ppEnabledLayerNames: *const *const ::std::os::raw::c_char,
        pub enabledExtensionCount: u32,
        pub ppEnabledExtensionNames: *const *const ::std::os::raw::c_char,
        pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkSwapchainCreateInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkSwapchainCreateFlagsKHR,
        pub surface: VkSurfaceKHR,
        pub minImageCount: u32,
        pub imageFormat: VkFormat,
        pub imageColorSpace: VkColorSpaceKHR,
        pub imageExtent: VkExtent2D,
        pub imageArrayLayers: u32,
        pub imageUsage: VkImageUsageFlags,
        pub imageSharingMode: VkSharingMode,
        pub queueFamilyIndexCount: u32,
        pub pQueueFamilyIndices: *const u32,
        pub preTransform: VkSurfaceTransformFlagBitsKHR,
        pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
        pub presentMode: VkPresentModeKHR,
        pub clipped: VkBool32,
        pub oldSwapchain: VkSwapchainKHR,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkCommandPoolCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkDeviceCreateFlags,
        pub queueFamilyIndex: u32,
    }
);

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
    pub struct VkImageViewCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkImageViewCreateFlags,
        pub image: VkImage,
        pub viewType: VkImageViewType,
        pub format: VkFormat,
        pub components: VkComponentMapping,
        pub subresourceRange: VkImageSubresourceRange,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkSamplerCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkSamplerCreateFlags,
        pub magFilter: VkFilter,
        pub minFilter: VkFilter,
        pub mipmapMode: VkSamplerMipmapMode,
        pub addressModeU: VkSamplerAddressMode,
        pub addressModeV: VkSamplerAddressMode,
        pub addressModeW: VkSamplerAddressMode,
        pub mipLodBias: f32,
        pub anisotropyEnable: VkBool32,
        pub maxAnisotropy: f32,
        pub compareEnable: VkBool32,
        pub compareOp: VkCompareOp,
        pub minLod: f32,
        pub maxLod: f32,
        pub borderColor: VkBorderColor,
        pub unnormalizedCoordinates: VkBool32,
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
    pub struct VkCommandBufferAllocateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub commandPool: VkCommandPool,
        pub level: VkCommandBufferLevel,
        pub commandBufferCount: u32,
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
    pub struct VkDescriptorSetAllocateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub descriptorPool: VkDescriptorPool,
        pub descriptorSetCount: u32,
        pub pSetLayouts: *const VkDescriptorSetLayout,
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
    pub struct VkSemaphoreCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkSemaphoreCreateFlags,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineVertexInputStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineVertexInputStateCreateFlags,
        pub vertexBindingDescriptionCount: u32,
        pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
        pub vertexAttributeDescriptionCount: u32,
        pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineInputAssemblyStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineInputAssemblyStateCreateFlags,
        pub topology: VkPrimitiveTopology,
        pub primitiveRestartEnable: VkBool32,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineViewportStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineViewportStateCreateFlags,
        pub viewportCount: u32,
        pub pViewports: *const VkViewport,
        pub scissorCount: u32,
        pub pScissors: *const VkRect2D,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineRasterizationStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineRasterizationStateCreateFlags,
        pub depthClampEnable: VkBool32,
        pub rasterizerDiscardEnable: VkBool32,
        pub polygonMode: VkPolygonMode,
        pub cullMode: VkCullModeFlags,
        pub frontFace: VkFrontFace,
        pub depthBiasEnable: VkBool32,
        pub depthBiasConstantFactor: f32,
        pub depthBiasClamp: f32,
        pub depthBiasSlopeFactor: f32,
        pub lineWidth: f32,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineColorBlendStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineColorBlendStateCreateFlags,
        pub logicOpEnable: VkBool32,
        pub logicOp: VkLogicOp,
        pub attachmentCount: u32,
        pub pAttachments: *const VkPipelineColorBlendAttachmentState,
        pub blendConstants: [f32; 4usize],
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineMultisampleStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineMultisampleStateCreateFlags,
        pub rasterizationSamples: VkSampleCountFlagBits,
        pub sampleShadingEnable: VkBool32,
        pub minSampleShading: f32,
        pub pSampleMask: *const VkSampleMask,
        pub alphaToCoverageEnable: VkBool32,
        pub alphaToOneEnable: VkBool32,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPipelineDynamicStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineDynamicStateCreateFlags,
        pub dynamicStateCount: u32,
        pub pDynamicStates: *const VkDynamicState,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkRenderPassCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkRenderPassCreateFlags,
        pub attachmentCount: u32,
        pub pAttachments: *const VkAttachmentDescription,
        pub subpassCount: u32,
        pub pSubpasses: *const VkSubpassDescription,
        pub dependencyCount: u32,
        pub pDependencies: *const VkSubpassDependency,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkRenderPassBeginInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub renderPass: VkRenderPass,
        pub framebuffer: VkFramebuffer,
        pub renderArea: VkRect2D,
        pub clearValueCount: u32,
        pub pClearValues: *const VkClearValue,
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
    pub struct VkGraphicsPipelineCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkPipelineCreateFlags,
        pub stageCount: u32,
        pub pStages: *const VkPipelineShaderStageCreateInfo,
        pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
        pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
        pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
        pub pViewportState: *const VkPipelineViewportStateCreateInfo,
        pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
        pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
        pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
        pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
        pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
        pub layout: VkPipelineLayout,
        pub renderPass: VkRenderPass,
        pub subpass: u32,
        pub basePipelineHandle: VkPipeline,
        pub basePipelineIndex: i32,
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
    pub struct VkFramebufferCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkFramebufferCreateFlags,
        pub renderPass: VkRenderPass,
        pub attachmentCount: u32,
        pub pAttachments: *const VkImageView,
        pub width: u32,
        pub height: u32,
        pub layers: u32,
    }
);

impl_builder_for_vk_structure_t!(
    pub struct VkPresentInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub waitSemaphoreCount: u32,
        pub pWaitSemaphores: *const VkSemaphore,
        pub swapchainCount: u32,
        pub pSwapchains: *const VkSwapchainKHR,
        pub pImageIndices: *const u32,
        pub pResults: *mut VkResult,
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

#[cfg(all(target_os = "windows", feature = "graphics"))]
impl_builder_for_vk_structure_t!(
    pub struct VkWin32SurfaceCreateInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const ::std::os::raw::c_void,
        pub flags: VkWin32SurfaceCreateFlagsKHR,
        pub hinstance: HINSTANCE,
        pub hwnd: HWND,
    }
);

impl_builder_for_vk_none_structure_t!(
    pub struct VkViewport {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
        pub minDepth: f32,
        pub maxDepth: f32,
    }
);

impl_builder_for_vk_none_structure_t!(
    pub struct VkPipelineColorBlendAttachmentState {
        pub blendEnable: VkBool32,
        pub srcColorBlendFactor: VkBlendFactor,
        pub dstColorBlendFactor: VkBlendFactor,
        pub colorBlendOp: VkBlendOp,
        pub srcAlphaBlendFactor: VkBlendFactor,
        pub dstAlphaBlendFactor: VkBlendFactor,
        pub alphaBlendOp: VkBlendOp,
        pub colorWriteMask: VkColorComponentFlags,
    }
);

impl_builder_for_vk_none_structure_t!(
    pub struct VkAttachmentDescription {
        pub flags: VkAttachmentDescriptionFlags,
        pub format: VkFormat,
        pub samples: VkSampleCountFlagBits,
        pub loadOp: VkAttachmentLoadOp,
        pub storeOp: VkAttachmentStoreOp,
        pub stencilLoadOp: VkAttachmentLoadOp,
        pub stencilStoreOp: VkAttachmentStoreOp,
        pub initialLayout: VkImageLayout,
        pub finalLayout: VkImageLayout,
    }
);

impl_builder_for_vk_none_structure_t!(
    pub struct VkAttachmentReference {
        pub attachment: u32,
        pub layout: VkImageLayout,
    }
);

impl_builder_for_vk_none_structure_t!(
    pub struct VkSubpassDescription {
        pub flags: VkSubpassDescriptionFlags,
        pub pipelineBindPoint: VkPipelineBindPoint,
        pub inputAttachmentCount: u32,
        pub pInputAttachments: *const VkAttachmentReference,
        pub colorAttachmentCount: u32,
        pub pColorAttachments: *const VkAttachmentReference,
        pub pResolveAttachments: *const VkAttachmentReference,
        pub pDepthStencilAttachment: *const VkAttachmentReference,
        pub preserveAttachmentCount: u32,
        pub pPreserveAttachments: *const u32,
    }
);

impl_builder_for_vk_none_structure_t!(
    pub struct VkSubpassDependency {
        pub srcSubpass: u32,
        pub dstSubpass: u32,
        pub srcStageMask: VkPipelineStageFlags,
        pub dstStageMask: VkPipelineStageFlags,
        pub srcAccessMask: VkAccessFlags,
        pub dstAccessMask: VkAccessFlags,
        pub dependencyFlags: VkDependencyFlags,
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
impl_default_for_vk_pointer_t!(VkSurfaceKHR_T);
impl_default_for_vk_pointer_t!(VkSwapchainKHR_T);