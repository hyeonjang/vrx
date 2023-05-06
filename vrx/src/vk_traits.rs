include!("vk_header.rs");

//
// OOP
// trait binding
//

macro_rules! create_func {
    // other things
    (DECLARE $name:expr $(, $khr:expr)?) => {
        paste! {
            fn [<create_ $name:snake>](
                &self,
                [<$name:snake _create_info>]: *const [<Vk $name CreateInfo $($khr)?>],
                p_allocator: Option<*const VkAllocationCallbacks>,
            ) -> [<Vk $name $($khr)?>];
        }
    };

    (DEFINE $name:expr $(, $khr:expr)?) => {
        paste! {
            fn [<create_ $name:snake>](
                &self,
                [<$name:snake _create_info>]: *const [<Vk $name CreateInfo $($khr)?>],
                p_allocator: Option<*const VkAllocationCallbacks>,
            ) -> [<Vk $name $($khr)?>] {
                let mut instance = vk_instantiate!([<Vk $name $($khr)?>]);

                unsafe {
                    if let Some(p) = p_allocator {
                        [<vkCreate $name $($khr)?>](*self, [<$name:snake _create_info>], p_allocator.unwrap(), &mut instance);
                    } else {
                        [<vkCreate $name $($khr)?>](*self, [<$name:snake _create_info>], null(), &mut instance);
                    }
                }
                instance
            }
        }
    };

    // pipeline cases
    (DC_PIPE $name:expr) => {
        paste! {
            // vkCreateComputePipelines(
            //             self.device,
            //             pipeline_cache,
            //             create_info_count,
            //             pipeline_create_infos,
            //             null(),
            //             pipelines.as_mut_ptr(),
            //         )
            fn [<create_ $name:snake s>](
                &self,
                pipeline_cache: VkPipelineCache,
                [<$name:snake _create_info>]: &[[<Vk $name CreateInfo>]],
                p_allocator: Option<*const VkAllocationCallbacks>,
            ) -> Vec<VkPipeline>;
        }
    };

    // pipeline cases
    (DF_PIPE $name:expr) => {
        paste! {
            fn [<create_ $name:snake s>](
                &self,
                pipeline_cache: VkPipelineCache,
                [<$name:snake _create_info>]: &[[<Vk $name CreateInfo>]],
                p_allocator: Option<*const VkAllocationCallbacks>,
            ) -> Vec<VkPipeline> {
                let size = [<$name:snake _create_info>].len();
                let mut pipelines = vec![vk_instantiate!(VkPipeline); size];

                unsafe {
                    if let Some(p) = p_allocator {
                        vk_assert([<vkCreate $name s>](
                            *self,
                            pipeline_cache,
                            size as u32,
                            [<$name:snake _create_info>].as_ptr(),
                            p_allocator.unwrap(),
                            pipelines.as_mut_ptr(),
                        ));
                    } else {
                        vk_assert([<vkCreate $name s>](
                            *self,
                            pipeline_cache,
                            size as u32,
                            [<$name:snake _create_info>].as_ptr(),
                            null(),
                            pipelines.as_mut_ptr(),
                        ));
                    }
                }

                pipelines
            }
        }
};
}

macro_rules! destroy_func {
    (DECLARE $name:expr $(, $khr:expr)?) => {
        paste! {
            fn [<destroy_ $name:snake>](
                &self,
                [<$name:snake>]: [<Vk $name $($khr)?>],
                p_allocator: Option<*const VkAllocationCallbacks>,
            );
        }
    };

    (DEFINE $name:expr $(, $khr:expr)?) => {
        paste! {
            fn [<destroy_ $name:snake>](
                &self,
                [<$name:snake>]: [<Vk $name $($khr)?>],
                p_allocator: Option<*const VkAllocationCallbacks>,
            ) {

                unsafe {
                    if let Some(p) = p_allocator {
                        [<vkDestroy $name $($khr)?>](*self, [<$name:snake>], p_allocator.unwrap());
                    } else {
                        [<vkDestroy $name $($khr)?>](*self, [<$name:snake>], null());
                    }
                }
            }
        }
    };
}

pub trait VkPhysicalDeviceFunctions {
    // pub fn getFeatures(&self) -> *mut VkPhyiscalDeviceFeatures;
    // pub fn getFormatProperties(&self, format: VkFormat) -> *mut VkFormatProperties;

    // 
    fn create_device(&self, create_info: *const VkDeviceCreateInfo, p_allocator: Option<*const VkAllocationCallbacks>) -> VkDevice;
}

impl VkPhysicalDeviceFunctions for VkPhysicalDevice {

    fn create_device(&self, create_info: *const VkDeviceCreateInfo, p_allocator: Option<*const VkAllocationCallbacks>) -> VkDevice {
        let mut device = vk_instantiate!(VkDevice);
        unsafe {
            if let Some(p) = p_allocator { 
                vkCreateDevice(
                    *self,
                    create_info,
                    p_allocator.unwrap(), 
                    &mut device,
                );
            } else {
                vkCreateDevice(
                    *self,
                    create_info,
                    null(), 
                    &mut device,
                );
            }
        }
        device
    }
}

pub trait VkDeviceFunctions {
    create_func!(DECLARE CommandPool);
    create_func!(DECLARE Buffer);
    create_func!(DECLARE Image);
    create_func!(DECLARE ImageView);
    create_func!(DECLARE Sampler);
    create_func!(DECLARE DescriptorPool);
    create_func!(DECLARE DescriptorSetLayout);
    create_func!(DECLARE Fence);
    create_func!(DECLARE Semaphore);
    create_func!(DECLARE Swapchain, KHR);
    create_func!(DECLARE RenderPass);
    create_func!(DECLARE Framebuffer);
    create_func!(DECLARE PipelineCache);
    create_func!(DECLARE PipelineLayout);
    create_func!(DC_PIPE ComputePipeline);
    create_func!(DC_PIPE GraphicsPipeline);

    destroy_func!(DECLARE CommandPool);
    destroy_func!(DECLARE Buffer);
    destroy_func!(DECLARE Image);
    destroy_func!(DECLARE ImageView);
    destroy_func!(DECLARE Sampler);
    destroy_func!(DECLARE DescriptorPool);
    destroy_func!(DECLARE DescriptorSetLayout);
    destroy_func!(DECLARE Fence);
    destroy_func!(DECLARE Semaphore);
    destroy_func!(DECLARE Swapchain, KHR);
    destroy_func!(DECLARE RenderPass);
    destroy_func!(DECLARE Framebuffer);
    destroy_func!(DECLARE PipelineCache);
    destroy_func!(DECLARE PipelineLayout);
    destroy_func!(DECLARE Pipeline);

    fn create_shader_module(
        &self,
        code: &[u8],
        p_allocator: Option<*const VkAllocationCallbacks>,
    ) -> VkShaderModule;
    destroy_func!(DECLARE ShaderModule);

    //
    // Control functions
    //
    // memory
    fn allocate_memory(&self, memory_allocate_info: *const VkMemoryAllocateInfo, p_allocator: Option<*const VkAllocationCallbacks>) -> VkDeviceMemory;
    fn map_memory(&self, offset: u64, size: u64, flags: u32, memory: &VkDeviceMemory) -> Result<*mut c_void>;
    fn unmap_memory(&self, memory: &VkDeviceMemory);
    fn free_memory(&self, memory: &VkDeviceMemory, p_allocator: Option<*const VkAllocationCallbacks>);

    fn bind_buffer_memory(
        &self,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize,
    ) -> VkResult;
    fn bind_image_memory(
        &self,
        image: VkImage,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize,
    ) -> VkResult;

    // command buffer
    fn allocate_command_buffers(
        &self,
        allocate_info: *const VkCommandBufferAllocateInfo,
    ) -> Vec<VkCommandBuffer>;
    // Queue
    fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> VkQueue;

    // Memory
    fn get_buffer_memory_requirements(&self, buffer: VkBuffer) -> VkMemoryRequirements;

    // Swapchain
    fn get_swapchain_images_khr(&self, swapchain: VkSwapchainKHR) -> Vec<VkImage>;
    fn acquire_next_image_khr(
        &self,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
    ) -> Result<u32, VkResult>;

    // Fence
    fn wait_for_fence(&self, fences: &[VkFence], wait_all: bool, timeout: u64);
    fn reset_fence(&self, fences: &[VkFence]);

    //
    fn wait_idle(&self) -> Result<VkResult>;
}

impl VkDeviceFunctions for VkDevice {
    create_func!(DEFINE CommandPool);
    create_func!(DEFINE Buffer);
    create_func!(DEFINE Image);
    create_func!(DEFINE ImageView);
    create_func!(DEFINE Sampler);
    create_func!(DEFINE DescriptorPool);
    create_func!(DEFINE DescriptorSetLayout);
    create_func!(DEFINE Fence);
    create_func!(DEFINE Semaphore);
    create_func!(DEFINE Swapchain, KHR);
    create_func!(DEFINE RenderPass);
    create_func!(DEFINE Framebuffer);
    create_func!(DEFINE PipelineCache);
    create_func!(DEFINE PipelineLayout);
    create_func!(DF_PIPE ComputePipeline);
    create_func!(DF_PIPE GraphicsPipeline);

    destroy_func!(DEFINE CommandPool);
    destroy_func!(DEFINE Buffer);
    destroy_func!(DEFINE Image);
    destroy_func!(DEFINE ImageView);
    destroy_func!(DEFINE Sampler);
    destroy_func!(DEFINE DescriptorPool);
    destroy_func!(DEFINE DescriptorSetLayout);
    destroy_func!(DEFINE Fence);
    destroy_func!(DEFINE Semaphore);
    destroy_func!(DEFINE Swapchain, KHR);
    destroy_func!(DEFINE RenderPass);
    destroy_func!(DEFINE Framebuffer);
    destroy_func!(DEFINE PipelineCache);
    destroy_func!(DEFINE PipelineLayout);
    destroy_func!(DEFINE Pipeline);

    fn create_shader_module(
        &self,
        code: &[u8],
        p_allocator: Option<*const VkAllocationCallbacks>,
    ) -> VkShaderModule {
        let mut module = vk_instantiate!(VkShaderModule);

        unsafe {
            let code = Vec::<u8>::from(code);
            let (prefix, code_u32, suffix) = code.align_to::<u32>();
            if !prefix.is_empty() || !suffix.is_empty() {
                // return Err(anyhow!("Load code for module failed"));
                return module;
            }

            let shader_create_info = VkShaderModuleCreateInfoBuilder::new()
                .code_size(code.len())
                .p_code(code_u32.as_ptr())
                .build();

            if let Some(p) = p_allocator {
                vk_assert(vkCreateShaderModule(
                    *self,
                    &shader_create_info,
                    p_allocator.unwrap(),
                    &mut module,
                ));
            } else {
                vk_assert(vkCreateShaderModule(
                    *self,
                    &shader_create_info,
                    null(),
                    &mut module,
                ));
            }
        }
        module
    }
    destroy_func!(DEFINE ShaderModule);

    //
    //  Control functions
    //
    // memory
    fn allocate_memory(&self, memory_allocate_info: *const VkMemoryAllocateInfo, p_allocator: Option<*const VkAllocationCallbacks>) -> VkDeviceMemory {
        let mut memory = vk_instantiate!(VkDeviceMemory);
        unsafe {
            if let Some(p) = p_allocator {
                vkAllocateMemory(
                    *self, 
                    memory_allocate_info, 
                    p_allocator.unwrap(),
                    &mut memory
                );
            } else {
                vkAllocateMemory(
                    *self,
                    memory_allocate_info, 
                    null(),
                    &mut memory
                );
            }
        }
        memory
    }

    fn map_memory(&self, offset: u64, size: u64, flags: u32, memory: &VkDeviceMemory) -> Result<*mut c_void> {
        unsafe {
            let mut mapped = MaybeUninit::<*mut c_void>::uninit();

            vk_assert(vkMapMemory(
                *self,
                *memory,
                offset,
                size,
                flags,
                mapped.as_mut_ptr(),
            ));

            Ok(mapped.assume_init())
        }
    }
    
    fn unmap_memory(&self, memory: &VkDeviceMemory) {
        unsafe {
            vkUnmapMemory(*self, *memory);
        }
    }

    fn free_memory(&self, memory: &VkDeviceMemory, p_allocator: Option<*const VkAllocationCallbacks>) {
        unsafe {
            if let Some(p) = p_allocator {
                vkFreeMemory(*self, *memory, p);
            } else {
                vkFreeMemory(*self, *memory, null());
            }
        }
    }

    fn bind_buffer_memory(
        &self,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize,
    ) -> VkResult {
        let mut result = VkResult::VK_SUCCESS;
        unsafe {
            result = vkBindBufferMemory(*self, buffer, memory, memory_offset);
        }
        result
    }

    fn bind_image_memory(
        &self,
        image: VkImage,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize,
    ) -> VkResult {
        let mut result = VkResult::VK_SUCCESS;
        unsafe {
            result = vkBindImageMemory(*self, image, memory, memory_offset);
        }
        result
    }

    // command buffer
    fn allocate_command_buffers(
        &self,
        allocate_info: *const VkCommandBufferAllocateInfo,
    ) -> Vec<VkCommandBuffer> {
        let size = unsafe { (*allocate_info).commandBufferCount };
        let mut command_buffers = vec![vk_instantiate!(VkCommandBuffer); size as usize];

        unsafe {
            vk_assert(vkAllocateCommandBuffers(
                *self,
                allocate_info,
                command_buffers.as_mut_ptr(),
            ));
        }
        command_buffers
    }

    // Queue
    fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> VkQueue {
        let mut queue = vk_instantiate!(VkQueue);
        unsafe {
            vkGetDeviceQueue(*self, queue_family_index, queue_index, &mut queue);
        };
        queue
    }

    // memory
    fn get_buffer_memory_requirements(&self, buffer: VkBuffer) -> VkMemoryRequirements {
        unsafe {
            let mut mem_req = VkMemoryRequirements {
                size: 0,
                alignment: 0,
                memoryTypeBits: 0,
            };
            vkGetBufferMemoryRequirements(*self, buffer, &mut mem_req);

            mem_req
        }
    }

    // Swapchain
    fn get_swapchain_images_khr(&self, swapchain: VkSwapchainKHR) -> Vec<VkImage> {
        // get images count
        let mut n_images: u32 = 0;
        unsafe {
            vk_assert(vkGetSwapchainImagesKHR(
                *self,
                swapchain,
                &mut n_images,
                null_mut(),
            ));
        }
        let mut images = vec![vk_instantiate!(VkImage); n_images as usize];
        unsafe {
            vk_assert(vkGetSwapchainImagesKHR(
                *self,
                swapchain,
                &mut n_images,
                images.as_mut_ptr(),
            ));
        }
        images
    }

    fn acquire_next_image_khr(
        &self,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
    ) -> Result<u32, VkResult> {
        let mut image_index = 0;
        let mut result = VkResult::VK_SUCCESS;
        unsafe {
            result = vkAcquireNextImageKHR(
                *self,
                swapchain,
                timeout,
                semaphore,
                fence,
                &mut image_index,
            );
        }

        let x: Result<u32, VkResult>;
        match result {
            VkResult::VK_SUCCESS => x = Result::Ok(image_index),
            _ => x = Err(result),
        }
        x
    }

    // Fence
    fn wait_for_fence(&self, fences: &[VkFence], wait_all: bool, timeout: u64) {
        unsafe {
            vk_assert(vkWaitForFences(
                *self,
                fences.len() as u32,
                fences.as_ptr(),
                wait_all as VkBool32,
                timeout,
            ));
        }
    }

    fn reset_fence(&self, fences: &[VkFence]) {
        unsafe {
            vkResetFences(*self, fences.len() as u32, fences.as_ptr());
        }
    }

    // wait
    fn wait_idle(&self) -> Result<VkResult> {
        let result: Result<VkResult>;
        unsafe {
            result = Ok(vkDeviceWaitIdle(*self));
        }
        result
    }
}

pub trait VkQueueFunctions {
    fn queue_submit(
        &self,
        index: usize,
        info_count: u32,
        infos: *const VkSubmitInfo,
        fence: VkFence,
    );
    fn queue_wait_idle(&self);
    fn queue_present_khr(&self, index: usize, present_info: &VkPresentInfoKHR) -> VkResult;
}

impl VkQueueFunctions for VkQueue {
    fn queue_submit(
        &self,
        index: usize,
        info_count: u32,
        infos: *const VkSubmitInfo,
        fence: VkFence,
    ) {
        unsafe {
            vk_assert(vkQueueSubmit(*self, info_count, infos, fence));
        }
    }

    fn queue_wait_idle(&self) {
        unsafe {
            vkQueueWaitIdle(*self);
        }
    }

    fn queue_present_khr(&self, index: usize, present_info: &VkPresentInfoKHR) -> VkResult {
        let mut result = VkResult::VK_SUCCESS;
        unsafe { result = vkQueuePresentKHR(*self, present_info) }
        result
    }
}