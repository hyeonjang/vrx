extern crate winit;
use vrx::vx::*;
use vrx::*;
use winit::{
    dpi::LogicalSize, event_loop::EventLoop, platform::windows::WindowExtWindows, window::Window,
    window::WindowBuilder,
};

const VERT_SPV: &[u8] = include_bytes!("./shader/vertex.spv");
const FRAG_SPV: &[u8] = include_bytes!("./shader/fragment.spv");

struct AppData {
    image_views: Vec<VkImageView>,
}

struct App {
    window: Window,
    device: vx::Device,
    surface: VkSurfaceKHR,
    swapchain: VkSwapchainKHR,
    framebuffers: Vec<VkFramebuffer>
}

impl App {
    pub fn new() -> Self {
        let window = Self::new_window(1024, 768);
        let device = vx::Device::new(&[(vx::QueueType::graphics, 1)]);
        let surface = Self::new_surface(&window);
        let swapchain = Self::new_swapchain(&device, &surface);

        Self {
            window: window,
            device: device,
            surface: surface,
            swapchain: swapchain,
        }
    }

    fn new_window(width: u32, height: u32) -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Vk test")
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();
        window
    }

    fn new_surface(window: &Window) -> VkSurfaceKHR {
        let mut CTX = vulkan_context();

        let win32_surface_create_info = VkWin32SurfaceCreateInfoKHRBuilder::new()
            .hinstance(window.hinstance() as vrx::HINSTANCE)
            .hwnd(window.hwnd() as vrx::HWND)
            .build();

        CTX.create_win32_surface_khr(&win32_surface_create_info, None)
    }

    fn new_swapchain(device: &vx::Device, surface: &VkSurfaceKHR) -> VkSwapchainKHR {
        let support = SwapchainSupport::new(*surface);
        let surface_format = support.get_swapchain_surface_format(
            VK_FORMAT_B8G8R8A8_SRGB,
            VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        );
        let present_mode = support.get_swapchain_present_mode(VK_PRESENT_MODE_MAILBOX_KHR);
        let extent = support.get_swapchain_extent();

        let swapchain_create_info = VkSwapchainCreateInfoKHRBuilder::new()
            .surface(*surface)
            .min_image_count(support.capabilities.minImageCount + 1)
            .image_format(surface_format.format)
            .image_color_space(surface_format.colorSpace)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32)
            .image_sharing_mode(VK_SHARING_MODE_EXCLUSIVE)
            .p_queue_family_indices(
                device
                    .queue_family_indices
                    .get(&vx::QueueType::graphics)
                    .unwrap()
                    .as_ptr(),
            )
            .pre_transform(support.capabilities.currentTransform)
            .present_mode(present_mode)
            .clipped(true as u32)
            .build();
        device
            .create_swapchain(&swapchain_create_info, None)
            .unwrap()
    }

    fn pipeline(&self) {
        //
        // 1. shader module
        //
        let vert = self.device.create_shader_module(VERT_SPV).unwrap();
        let frag = self.device.create_shader_module(FRAG_SPV).unwrap();

        let vert_stage = VkPipelineShaderStageCreateInfoBuilder::new()
            .stage(VK_SHADER_STAGE_VERTEX_BIT)
            .module(vert)
            .p_name(b"main\0".as_ptr() as *const i8)
            .build();
        let frag_stage = VkPipelineShaderStageCreateInfoBuilder::new()
            .stage(VK_SHADER_STAGE_VERTEX_BIT)
            .module(frag)
            .p_name(b"main\0".as_ptr() as *const i8)
            .build();

        //
        // 2. fixed function
        //
        let input_assembly_state = VkPipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST)
            .primitive_restart_enable(false as u32)
            .build();

        let viewport = VkViewport {
            x: 0.0,
            y: 0.0,
            width: 1024.0,
            height: 768.0,
            minDepth: 0.0,
            maxDepth: 1.0,
        };

        let scissor = VkRect2D {
            offset: VkOffset{ 0.0, 0.0 },
            extent: todo!(),
        };

        let viewports = &[viewport];
        let scissors = &[scissor];
        let pipeline_viewport_state = VkPipelineViewportStateCreateInfoBuilder::new()
            .viewport_count(1)
            .p_viewports(viewports.as_ptr())
            .scissor_count(1)
            .p_scissors(scissors.as_ptr())
            .build();

        let pipeline_rasterization_state = VkPipelineRasterizationStateCreateInfoBuilder::new()
            .depth_clamp_enable(false as u32)
            .polygon_mode(VK_POLYGON_MODE_FILL)
            .line_width(1.0)
            .cull_mode(VK_CULL_MODE_FRONT_BIT as u32)
            .front_face(VK_FRONT_FACE_CLOCKWISE)
            .depth_bias_enable(false as u32)
            .build();

        let pipeline_multisample_state = VkPipelineMultisampleStateCreateInfoBuilder::new()
            .sample_shading_enable(false as u32)
            .rasterization_samples(VK_SAMPLE_COUNT_1_BIT)
            .build();

        let pipeline_color_blend_attachment_state =
            VkPipelineColorBlendAttachmentStateBuilder::new()
                .color_write_mask(VK_COLOR_COMPONENT_ALL_BIT as u32)
                .blend_enable(false as u32)
                .src_color_blend_factor(VK_BLEND_FACTOR_ONE)
                .dst_color_blend_factor(VK_BLEND_FACTOR_ZERO)
                .color_blend_op(VK_BLEND_OP_ADD)
                .src_alpha_blend_factor(VK_BLEND_FACTOR_ONE)
                .dst_alpha_blend_factor(VK_BLEND_FACTOR_ZERO)
                .alpha_blend_op(VK_BLEND_OP_ADD)
                .build();

        let attachments = &[pipeline_color_blend_attachment_state];
        let pipeline_color_blend_state = VkPipelineColorBlendStateCreateInfoBuilder::new()
            .logic_op_enable(false as u32)
            .logic_op(VK_LOGIC_OP_COPY)
            .attachment_count(attachments.len() as u32)
            .p_attachments(attachments.as_ptr())
            .blend_constants([0.0, 0.0, 0.0, 0.0])
            .build();

        let dynamic_states = &[VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_LINE_WIDTH];

        let dynamic_state = VkPipelineDynamicStateCreateInfoBuilder::new()
            .dynamic_state_count(dynamic_states.len() as u32)
            .p_dynamic_states(dynamic_states.as_ptr())
            .build();

        let pipeline_layout_create_info = VkPipelineLayoutCreateInfoBuilder::new().build();

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

        // attachments
        let color_attachment_description = VkAttachmentDescriptionBuilder::new()
            // .format()
            .samples(VK_SAMPLE_COUNT_1_BIT)
            .load_op(VK_ATTACHMENT_LOAD_OP_CLEAR)
            .store_op(VK_ATTACHMENT_STORE_OP_STORE)
            .stencil_load_op(VK_ATTACHMENT_LOAD_OP_DONT_CARE)
            .stencil_store_op(VK_ATTACHMENT_LOAD_OP_DONT_CARE)
            .initial_layout(VK_IMAGE_LAYOUT_UNDEFINED)
            .final_layout(VK_IMAGE_LAYOUT_PRESENT_SRC_KHR)
            .build();

        let attachments = &[color_attachment_description];
        let subpasses = &[subpass];
        let render_pass_create_info = VkRenderPassCreateInfoBuilder::new()
            .attachment_count(attachments.len() as u32)
            .p_attachments(attachments.as_ptr())
            .subpass_count(subpasses.len() as u32)
            .p_subpasses(subpasses.as_ptr())
            .build();

        let render_pass = self
            .device
            .create_render_pass(&render_pass_create_info, None);

        // let graphics_pipeline_create_info = VkGraphicsPipelineCreateInfoBuilder::new().build();

        // let graphics_pipeline = self
            // .device
            // .create_graphics_pipeline(&graphics_pipeline_create_info, None);
    }

    fn frame_buffers(&self) {

    }
}

fn main() {
    println!("Hello, world!");

    let app = App::new();
    app.pipeline();

    // let image_view = device.create_image_view(image_view_create_info, None);
}
