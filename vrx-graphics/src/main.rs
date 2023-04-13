extern crate winit;
use anyhow::{anyhow, Result};
use paste::paste;
use vrx::vx::*;
use vrx::*;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    platform::windows::WindowExtWindows,
    window::Window,
    window::WindowBuilder,
};

#[derive(Debug)]
struct Presentation<'a> {
    surface: VkSurfaceKHR,
    swapchain: VkSwapchainKHR,
    images: Vec<VkImage>,
    image_views: Vec<VkImageView>,
    format: VkSurfaceFormatKHR,
    extent: VkExtent2D,

    device: Option<&'a vx::Device>,
}

impl<'a> Default for Presentation<'a> {
    fn default() -> Self {
        Self {
            surface: vk_instantiate!(VkSurfaceKHR),
            swapchain: vk_instantiate!(VkSwapchainKHR),
            images: vec![],
            image_views: vec![],
            format: VkSurfaceFormatKHR::default(),
            extent: VkExtent2D::default(),
            device: None,
        }
    }
}

impl<'a> Presentation<'a> {
    fn new(device: &'a vx::Device, window: &Window) -> Self {
        let surface = Self::new_surface(&window);
        let support = SwapchainSupport::new(&surface);

        let format = support.get_swapchain_surface_format(
            VK_FORMAT_R8G8B8A8_SRGB,
            VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        );
        let present_mode = support.get_swapchain_present_mode(VK_PRESENT_MODE_MAILBOX_KHR);
        let extent = support.get_swapchain_extent();
        let swapchain = Self::new_swapchain(
            device,
            &surface,
            support.capabilities,
            format,
            present_mode,
            extent,
        );

        let images = device.get_swapchain_images_khr(swapchain);
        let image_views = Self::new_image_views(device, &images, format);

        let device = Some(device);
        Self {
            surface,
            swapchain,
            images,
            image_views,
            format,
            extent,
            device,
        }
    }

    fn new_surface(window: &Window) -> VkSurfaceKHR {
        let ctx = vulkan_context();

        let win32_surface_create_info = VkWin32SurfaceCreateInfoKHRBuilder::new()
            .hinstance(window.hinstance() as vrx::HINSTANCE)
            .hwnd(window.hwnd() as vrx::HWND)
            .build();

        ctx.create_win32_surface_khr(&win32_surface_create_info, None)
    }

    fn new_swapchain(
        device: &vx::Device,
        surface: &VkSurfaceKHR,
        capabilities: VkSurfaceCapabilitiesKHR,
        surface_format: VkSurfaceFormatKHR,
        present_mode: VkPresentModeKHR,
        extent: VkExtent2D,
    ) -> VkSwapchainKHR {
        let queue_family_indices = device
            .queue_family_indices
            .get(&vx::QueueType::graphics)
            .unwrap();
        let swapchain_create_info = VkSwapchainCreateInfoKHRBuilder::new()
            .surface(*surface)
            .min_image_count(capabilities.minImageCount)
            .image_format(surface_format.format)
            .image_color_space(surface_format.colorSpace)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32)
            .image_sharing_mode(VK_SHARING_MODE_EXCLUSIVE)
            .queue_family_index_count(queue_family_indices.len() as u32)
            .p_queue_family_indices(queue_family_indices.as_ptr())
            .pre_transform(capabilities.currentTransform)
            .composite_alpha(VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR)
            .present_mode(present_mode)
            .clipped(VK_TRUE)
            .build();
        let swapchain = device
            .create_swapchain(&swapchain_create_info, None)
            .unwrap();
        swapchain
    }

    fn new_image_views(
        device: &vx::Device,
        images: &Vec<VkImage>,
        format: VkSurfaceFormatKHR,
    ) -> Vec<VkImageView> {
        let image_views: Vec<_> = images
            .iter()
            .map(|image| {
                let components = VkComponentMapping {
                    r: VK_COMPONENT_SWIZZLE_IDENTITY,
                    g: VK_COMPONENT_SWIZZLE_IDENTITY,
                    b: VK_COMPONENT_SWIZZLE_IDENTITY,
                    a: VK_COMPONENT_SWIZZLE_IDENTITY,
                };

                let subresource_range = VkImageSubresourceRange {
                    aspectMask: VK_IMAGE_ASPECT_COLOR_BIT as u32,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                };

                let image_view_create_info = VkImageViewCreateInfoBuilder::new()
                    .image(*image)
                    .view_type(VK_IMAGE_VIEW_TYPE_2D)
                    .format(format.format)
                    .components(components)
                    .subresource_range(subresource_range)
                    .build();
                device
                    .create_image_view(&image_view_create_info, None)
                    .unwrap()
            })
            .collect::<Vec<VkImageView>>();

        image_views
    }

    pub fn destroy(&mut self) {
        // self.images
        //     .iter()
        //     .for_each(|i| self.device.unwrap().destroy_image(*i, None));

        self.image_views
            .iter()
            .for_each(|iv| self.device.unwrap().destroy_image_view(*iv, None));

        self.device.unwrap().destroy_swapchain(self.swapchain, None);
    }
}

#[derive(Debug)]
struct ShaderModules<'a> {
    shader_modules: Vec<VkShaderModule>,
    shader_stages: Vec<VkShaderStageFlagBits>,
    create_infos: Vec<VkPipelineShaderStageCreateInfo>,

    device: Option<&'a vx::Device>,
}

impl<'a> Default for ShaderModules<'a> {
    fn default() -> Self {
        Self {
            shader_modules: vec![],
            shader_stages: vec![],
            create_infos: vec![],

            device: None,
        }
    }
}

impl<'a> ShaderModules<'a> {
    fn new(
        device: &'a vx::Device,
        shader_bytes: &[&[u8]],
        shader_stages: &[VkShaderStageFlagBits],
    ) -> Self {
        let shader_modules = shader_bytes
            .iter()
            .enumerate()
            .map(|(i, bytes)| device.create_shader_module(bytes).unwrap())
            .collect::<Vec<VkShaderModule>>();

        let mut shader_modules = Self {
            device: Some(device),
            shader_modules: shader_modules,
            shader_stages: shader_stages.to_vec(),
            create_infos: vec![],
        };

        shader_modules.create_shader_stage_create_info();
        shader_modules
    }

    fn destroy(&mut self) {
        self.shader_modules
            .iter()
            .for_each(|m| self.device.unwrap().destroy_shader_module(*m, None));
    }

    fn create_shader_stage_create_info(&mut self) {
        let create_infos = self
            .shader_modules
            .iter()
            .enumerate()
            .map(|(i, module)| {
                VkPipelineShaderStageCreateInfoBuilder::new()
                    .stage(self.shader_stages[i])
                    .module(*module)
                    .p_name(b"main\0".as_ptr() as *const i8)
                    .build()
            })
            .collect::<Vec<VkPipelineShaderStageCreateInfo>>();
        self.create_infos = create_infos;
    }

    fn len(&self) -> usize {
        self.shader_modules.len()
    }

    fn create_infos_ptr(&self) -> *const VkPipelineShaderStageCreateInfo {
        self.create_infos.as_ptr()
    }
}

#[derive(Debug, Default)]
struct GraphicsPipelineProperties {
    // shader_stages: Vec<VkPipelineShaderStageCreateInfo>,
    // shader_stages: ShaderModules<'a>,
    vertex_input_state: VkPipelineVertexInputStateCreateInfo,
    input_assembly_state: VkPipelineInputAssemblyStateCreateInfo,
    viewport_state: VkPipelineViewportStateCreateInfo,
    rasterization_state: VkPipelineRasterizationStateCreateInfo,
    multisample_state: VkPipelineMultisampleStateCreateInfo,
    color_blend_state: VkPipelineColorBlendStateCreateInfo,
    viewports: Vec<VkViewport>,
    scissors: Vec<VkRect2D>,
    color_blend_attachments: Vec<VkPipelineColorBlendAttachmentState>,
}

const VERT_SPV: &[u8] = include_bytes!("./shader/vertex.spv");
const FRAG_SPV: &[u8] = include_bytes!("./shader/fragment.spv");

impl GraphicsPipelineProperties {
    fn new(device: &vx::Device, presentation: &Presentation) -> Self {
        //
        // 1. shader module
        //
        // let vert = device.create_shader_module(VERT_SPV).unwrap();
        // let frag = device.create_shader_module(FRAG_SPV).unwrap();

        // let vert_stage = VkPipelineShaderStageCreateInfoBuilder::new()
        //     .stage(VK_SHADER_STAGE_VERTEX_BIT)
        //     .module(vert)
        //     .p_name(b"main\0".as_ptr() as *const i8)
        //     .build();
        // let frag_stage = VkPipelineShaderStageCreateInfoBuilder::new()
        //     .stage(VK_SHADER_STAGE_FRAGMENT_BIT)
        //     .module(frag)
        //     .p_name(b"main\0".as_ptr() as *const i8)
        //     .build();
        // let shader_stages = vec![vert_stage, frag_stage];

        //
        // 2. fixed function
        //
        let vertex_input_state = VkPipelineVertexInputStateCreateInfoBuilder::new().build();

        let input_assembly_state = VkPipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST)
            .primitive_restart_enable(VK_FALSE)
            .build();

        let scissor = VkRect2D {
            offset: VkOffset2D { x: 0, y: 0 },
            extent: presentation.extent,
        };

        let viewports = VkViewport {
            x: 0.0,
            y: 0.0,
            width: presentation.extent.width as f32,
            height: presentation.extent.height as f32,
            minDepth: 0.0,
            maxDepth: 1.0,
        };

        let viewports = vec![viewports];
        let scissors = vec![scissor];
        let viewport_state = VkPipelineViewportStateCreateInfoBuilder::new()
            .viewport_count(viewports.len() as u32)
            .p_viewports(viewports.as_ptr())
            .scissor_count(scissors.len() as u32)
            .p_scissors(scissors.as_ptr())
            .build();

        let rasterization_state = VkPipelineRasterizationStateCreateInfoBuilder::new()
            .depth_clamp_enable(VK_FALSE)
            .rasterizer_discard_enable(VK_FALSE)
            .polygon_mode(VK_POLYGON_MODE_FILL)
            .line_width(1.0)
            .cull_mode(VK_CULL_MODE_NONE as u32)
            .front_face(VK_FRONT_FACE_CLOCKWISE)
            .depth_bias_enable(VK_FALSE)
            .build();

        let multisample_state = VkPipelineMultisampleStateCreateInfoBuilder::new()
            .sample_shading_enable(VK_FALSE)
            .rasterization_samples(VK_SAMPLE_COUNT_1_BIT)
            .build();

        let color_blend_attachment_state = VkPipelineColorBlendAttachmentStateBuilder::new()
            .color_write_mask(VK_COLOR_COMPONENT_ALL_BIT as u32)
            .blend_enable(VK_FALSE)
            .src_color_blend_factor(VK_BLEND_FACTOR_ONE)
            .dst_color_blend_factor(VK_BLEND_FACTOR_ZERO)
            .color_blend_op(VK_BLEND_OP_ADD)
            .src_alpha_blend_factor(VK_BLEND_FACTOR_ONE)
            .dst_alpha_blend_factor(VK_BLEND_FACTOR_ZERO)
            .alpha_blend_op(VK_BLEND_OP_ADD)
            .build();

        let color_blend_attachments = vec![color_blend_attachment_state];
        let color_blend_state = VkPipelineColorBlendStateCreateInfoBuilder::new()
            .logic_op_enable(VK_FALSE)
            .logic_op(VK_LOGIC_OP_COPY)
            .attachment_count(color_blend_attachments.len() as u32)
            .p_attachments(color_blend_attachments.as_ptr())
            .blend_constants([0.0, 0.0, 0.0, 1.0])
            .build();

        Self {
            vertex_input_state,
            input_assembly_state,
            viewport_state,
            rasterization_state,
            multisample_state,
            color_blend_state,
            viewports,
            scissors,
            color_blend_attachments,
        }
    }
}

struct GraphicsPipeline<'a> {
    render_pass: VkRenderPass,
    pipeline_layout: VkPipelineLayout,
    pipeline: VkPipeline,
    device: Option<&'a vx::Device>,
}

impl<'a> Default for GraphicsPipeline<'a> {
    fn default() -> Self {
        Self {
            render_pass: vk_instantiate!(VkRenderPass),
            pipeline_layout: vk_instantiate!(VkPipelineLayout),
            pipeline: vk_instantiate!(VkPipeline),
            device: None,
        }
    }
}

impl<'a> GraphicsPipeline<'a> {
    fn new(
        device: &'a vx::Device,
        presentation: &Presentation,
        shader_stages: &ShaderModules,
        properties: &GraphicsPipelineProperties,
    ) -> Self {
        let mut instance = Self::default();
        instance.set_device(device);

        // real create
        instance.create_render_pass(presentation);
        instance.create_pipeline_layout();
        instance.create_pipeline(shader_stages, properties);

        instance
    }

    fn set_device(&mut self, device: &'a vx::Device) {
        self.device = Some(device);
    }

    pub fn destroy(&mut self) {
        self.device.unwrap().destroy_pipeline(self.pipeline, None);
        self.device
            .unwrap()
            .destroy_pipeline_layout(self.pipeline_layout, None);
        self.device
            .unwrap()
            .destroy_render_pass(self.render_pass, None);
    }

    fn create_render_pass(&mut self, presentation: &Presentation) {
        // render pass
        // attachments
        let color_attachment_description = VkAttachmentDescriptionBuilder::new()
            .format(presentation.format.format)
            .samples(VK_SAMPLE_COUNT_1_BIT)
            .load_op(VK_ATTACHMENT_LOAD_OP_CLEAR)
            .store_op(VK_ATTACHMENT_STORE_OP_STORE)
            .stencil_load_op(VK_ATTACHMENT_LOAD_OP_CLEAR)
            .stencil_store_op(VK_ATTACHMENT_STORE_OP_DONT_CARE)
            .initial_layout(VK_IMAGE_LAYOUT_UNDEFINED)
            .final_layout(VK_IMAGE_LAYOUT_PRESENT_SRC_KHR)
            .build();

        // subpass
        let color_attachment_ref = VkAttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL)
            .build();
        let color_attachments = &[color_attachment_ref];
        let subpass = VkSubpassDescriptionBuilder::new()
            .pipeline_bind_point(VK_PIPELINE_BIND_POINT_GRAPHICS)
            .color_attachment_count(color_attachments.len() as u32)
            .p_color_attachments(color_attachments.as_ptr())
            .build();

        // dependencies
        let dependency = VkSubpassDependencyBuilder::new()
            .src_subpass(VK_SUBPASS_EXTERNAL as u32)
            .dst_subpass(0)
            .src_stage_mask(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32)
            .src_access_mask(0)
            .dst_stage_mask(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32)
            .dst_access_mask(VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32)
            .build();

        let attachments = &[color_attachment_description];
        let subpasses = &[subpass];
        let dependencies = &[dependency];
        let render_pass_create_info = VkRenderPassCreateInfoBuilder::new()
            .attachment_count(attachments.len() as u32)
            .p_attachments(attachments.as_ptr())
            .subpass_count(subpasses.len() as u32)
            .p_subpasses(subpasses.as_ptr())
            .dependency_count(dependencies.len() as u32)
            .p_dependencies(dependencies.as_ptr())
            .build();

        self.render_pass = self
            .device
            .unwrap()
            .create_render_pass(&render_pass_create_info, None)
            .unwrap();
    }

    fn create_pipeline_layout(&mut self) {
        let pipeline_layout_create_info = VkPipelineLayoutCreateInfoBuilder::new().build();
        self.pipeline_layout = self
            .device
            .unwrap()
            .create_pipeline_layout(&pipeline_layout_create_info, None)
            .unwrap();
    }

    fn create_pipeline(
        &mut self,
        shader_stages: &ShaderModules,
        properties: &GraphicsPipelineProperties,
    ) {
        let pipeline_create_info = VkGraphicsPipelineCreateInfoBuilder::new()
            .stage_count(shader_stages.len() as u32)
            .p_stages(shader_stages.create_infos_ptr())
            .p_vertex_input_state(&properties.vertex_input_state)
            .p_input_assembly_state(&properties.input_assembly_state)
            .p_viewport_state(&properties.viewport_state)
            .p_rasterization_state(&properties.rasterization_state)
            .p_multisample_state(&properties.multisample_state)
            .p_color_blend_state(&properties.color_blend_state)
            .layout(self.pipeline_layout)
            .render_pass(self.render_pass)
            .subpass(0)
            .build();

        let pipeline_cache_create_info = VkPipelineCacheCreateInfoBuilder::new().build();
        let pipeline_cache = self
            .device
            .unwrap()
            .create_pipeline_cache(&pipeline_cache_create_info, None)
            .unwrap();

        self.pipeline = self
            .device
            .unwrap()
            .create_graphics_pipelines(pipeline_cache, 1, &pipeline_create_info)
            .unwrap()[0];
    }
}

struct App<'a> {
    presentation: Presentation<'a>,
    shader_stages: ShaderModules<'a>,
    graphics_pipeline_properties: GraphicsPipelineProperties,
    graphics_pipeline: GraphicsPipeline<'a>,
    framebuffers: Vec<VkFramebuffer>,
    command_buffers: Vec<VkCommandBuffer>,
    image_available_semaphores: Vec<VkSemaphore>,
    render_finished_semaphores: Vec<VkSemaphore>,
    in_flight_fences: Vec<VkFence>,
    images_in_flight: Vec<VkFence>,
    frame: usize,
    resized: bool,
    device: &'a vx::Device,
}

// impl Drop for App {
//     fn drop(&mut self) {
//         // presentation.drop();

//         self.framebuffers
//             .iter()
//             .for_each(|buf| self.device.destroy_framebuffer(*buf, None));
//     }
// }

impl<'a> App<'a> {
    pub fn new(device: &'a vx::Device, window: &Window) -> App<'a> {
        let shader_stages = ShaderModules::new(
            device,
            &[VERT_SPV, FRAG_SPV],
            &[VK_SHADER_STAGE_VERTEX_BIT, VK_SHADER_STAGE_FRAGMENT_BIT],
        );
        let presentation = Presentation::new(device, window);
        let graphics_pipeline_properties = GraphicsPipelineProperties::new(device, &presentation);
        let graphics_pipeline = GraphicsPipeline::new(
            device,
            &presentation,
            &shader_stages,
            &graphics_pipeline_properties,
        );

        let mut app = Self {
            device: device,
            shader_stages: shader_stages,
            presentation: presentation,
            graphics_pipeline_properties: graphics_pipeline_properties,
            graphics_pipeline: graphics_pipeline,
            framebuffers: vec![],
            command_buffers: vec![],
            image_available_semaphores: vec![],
            render_finished_semaphores: vec![],
            in_flight_fences: vec![],
            images_in_flight: vec![],
            frame: 0,
            resized: false,
        };

        app.create_framebuffers();
        app.create_command_buffers();
        app.create_sync_objects();

        app
    }

    pub fn render(&mut self, window: &Window) -> Result<()> {
        // syn cpu gpu

        let in_flight_fence = self.in_flight_fences[self.frame];

        self.device
            .wait_for_fence(1, &in_flight_fence, true, u64::MAX);

        let result = self.device.acquire_next_image_khr(
            self.presentation.swapchain,
            u64::MAX,
            self.image_available_semaphores[self.frame],
            std::ptr::null_mut(),
        );

        let image_index = match result {
            Ok(image_index) => image_index,
            Err(VkResult::VK_ERROR_OUT_OF_DATE_KHR) => return self.recreate_presentation(window),
            Err(e) => return Err(anyhow!("{:?}", e)),
        };

        let image_in_flight = self.images_in_flight[image_index as usize];
        // println!("{:?}, {:?}", self.frame, self.images_in_flight[image_index as usize]);
        if !image_in_flight.is_null() {
            self.device
                .wait_for_fence(1, &image_in_flight, true, u64::MAX);
        }
        self.images_in_flight[image_index as usize] = in_flight_fence;

        let wait_dst_stage_mask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as u32;
        let wait_semaphores = &[self.image_available_semaphores[self.frame]];
        let command_buffer = self.command_buffers[image_index as usize];
        let signal_semaphores = &[self.image_available_semaphores[self.frame]];
        let submit_info = VkSubmitInfoBuilder::new()
            .wait_semaphore_count(wait_semaphores.len() as u32)
            .p_wait_semaphores(wait_semaphores.as_ptr())
            .p_wait_dst_stage_mask(&wait_dst_stage_mask)
            .command_buffer_count(1)
            .p_command_buffers(&command_buffer)
            .signal_semaphore_count(signal_semaphores.len() as u32)
            .p_signal_semaphores(signal_semaphores.as_ptr())
            .build();

        self.device.reset_fence(
            self.in_flight_fences.len() as u32,
            self.in_flight_fences.as_ptr(),
        );

        self.device.queue_submit(
            QueueType::graphics,
            0,
            1,
            &submit_info,
            self.in_flight_fences[self.frame],
        );

        // presenting queue
        let present_info = VkPresentInfoKHRBuilder::new()
            .wait_semaphore_count(signal_semaphores.len() as u32)
            .p_wait_semaphores(signal_semaphores.as_ptr())
            .swapchain_count(1)
            .p_swapchains(&self.presentation.swapchain)
            .p_image_indices(&image_index)
            .build();
        let result = self
            .device
            .queue_present_khr(QueueType::graphics, 0, &present_info);

        let changed =
            result == VkResult::VK_SUBOPTIMAL_KHR || result == VkResult::VK_ERROR_OUT_OF_DATE_KHR;

        if changed {
            self.recreate_presentation(window);
        } else {
            return Ok(());
        }

        // synchronization
        self.frame = (self.frame + 1) % 2;

        Ok(())
    }

    fn create_framebuffers(&mut self) {
        self.framebuffers = self
            .presentation
            .image_views
            .iter()
            .map(|image| {
                let framebuffer_create_info = VkFramebufferCreateInfoBuilder::new()
                    .render_pass(self.graphics_pipeline.render_pass)
                    .attachment_count(1)
                    .p_attachments(image)
                    .width(self.presentation.extent.width)
                    .height(self.presentation.extent.height)
                    .layers(1)
                    .build();
                self.device
                    .create_framebuffer(&framebuffer_create_info, None)
                    .unwrap()
            })
            .collect();
    }

    fn create_command_buffers(&mut self) {
        self.command_buffers = self
            .device
            .allocate_command_buffers(
                QueueType::graphics,
                VK_COMMAND_BUFFER_LEVEL_PRIMARY,
                self.framebuffers.len() as u32,
            )
            .unwrap();

        self.command_buffers.iter().enumerate().for_each(|(i, cmd)| {
            vkCmdBlock! {
                THIS *cmd;

                let render_area = VkRect2D { offset: VkOffset2D { x: 0, y: 0 }, extent: self.presentation.extent };
                let color_clear_value = VkClearValue { color: VkClearColorValue { float32:[0.0, 0.0, 1.0, 0.0] } };
                let clear_values = &[color_clear_value];

                let render_pass_begin_info = VkRenderPassBeginInfoBuilder::new()
                    .render_pass(self.graphics_pipeline.render_pass)
                    .render_area(render_area)
                    .clear_value_count(clear_values.len() as u32)
                    .p_clear_values(clear_values.as_ptr())
                    .framebuffer(self.framebuffers[i])
                    .build();

                BEGIN_RENDER_PASS(&render_pass_begin_info, VK_SUBPASS_CONTENTS_INLINE);
                BIND_PIPELINE(
                    VK_PIPELINE_BIND_POINT_GRAPHICS, self.graphics_pipeline.pipeline
                );
                DRAW(3, 1, 0, 0);
                END_RENDER_PASS();
            };
        });
    }

    fn create_sync_objects(&mut self) {
        let semaphore_create_info = VkSemaphoreCreateInfoBuilder::new().build();
        let fence_create_info = VkFenceCreateInfoBuilder::new()
            .flags(VK_FENCE_CREATE_SIGNALED_BIT as u32)
            .build();
        self.image_available_semaphores = vec![];
        self.render_finished_semaphores = vec![];
        self.in_flight_fences = vec![];
        for _ in 0..2 {
            self.image_available_semaphores.push(
                self.device
                    .create_semaphore(&semaphore_create_info, None)
                    .unwrap(),
            );
            self.render_finished_semaphores.push(
                self.device
                    .create_semaphore(&semaphore_create_info, None)
                    .unwrap(),
            );
            self.in_flight_fences
                .push(self.device.create_fence(&fence_create_info, None).unwrap());
        }

        self.images_in_flight = self
            .presentation
            .images
            .iter()
            .map(|_| std::ptr::null_mut())
            .collect();
    }

    fn recreate_presentation(&mut self, window: &Window) -> Result<()> {
        self.device.wait_idle();
        self.presentation.destroy();
        self.graphics_pipeline.destroy();

        self.presentation = Presentation::new(&self.device, window);
        self.graphics_pipeline_properties =
        GraphicsPipelineProperties::new(&self.device, &self.presentation);
        self.graphics_pipeline = GraphicsPipeline::new(
            &self.device,
            &self.presentation,
            &self.shader_stages,
            &self.graphics_pipeline_properties,
        );

        self.create_framebuffers();
        self.create_command_buffers();
        self.create_sync_objects();

        self.images_in_flight
            .resize(self.presentation.images.len(), std::ptr::null_mut());

        Ok(())
    }

    fn destroy_sync_objects(&mut self) {
        self.in_flight_fences.iter().for_each(|fence| {
            self.device.destroy_fence(*fence, None);
        });
        self.images_in_flight.iter().for_each(|fence| {
            self.device.destroy_fence(*fence, None);
        });
        self.image_available_semaphores
            .iter()
            .for_each(|semaphore| {
                self.device.destroy_semaphore(*semaphore, None);
            });
        self.render_finished_semaphores
            .iter()
            .for_each(|semaphore| {
                self.device.destroy_semaphore(*semaphore, None);
            });
    }

    pub fn destroy(&mut self) {
        self.device.wait_idle();

        self.presentation.destroy();
        self.shader_stages.destroy();
        self.graphics_pipeline.destroy();
        // self.destroy_sync_objects();
        // self.device.destroy();
    }
}

fn main() {
    println!("Hello, world!");

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello triangles")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)
        .unwrap();

    let device = vx::Device::new(&[(vx::QueueType::graphics, 1)]);
    let mut app = App::new(&device, &window);

    // non static event loop
    let mut destroying = false;
    let mut minimized = false;
    let some = event_loop.run_return(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Poll;
        control_flow.set_poll();
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared => { app.render(&window) }.unwrap(),
            // resize
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if size.width == 0 || size.height == 0 {
                    minimized = true;
                } else {
                    minimized = false;
                    app.resized = true;
                }
            }

            // Destroy our Vulkan app.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe {
                    app.destroy();
                }
            }
            _ => {}
        }
    });
}
