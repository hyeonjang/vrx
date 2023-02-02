use anyhow::*;
use std::env;
use std::ffi::*;
use std::ptr::*;
use vkcholesky::*;

const COMP_SPV: &[u8] = include_bytes!("./shader/cholesky.spv");

fn main() -> Result<()> {
    let vk_layer_path = env::var("VULKAN_SDK").unwrap();
    println!("{:?}", vk_layer_path);

    let device = Device::new();
    let descriptor = Descriptor::new(1, &device.self_);

    let data = vec![1, 2, 3, 4, 5];
    let mut buffer = device
        .create_buffer(
            data,
            (VK_BUFFER_USAGE_STORAGE_BUFFER_BIT
                | VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                | VK_BUFFER_USAGE_TRANSFER_DST_BIT) as u32,
            0,
        )
        .unwrap();
    buffer.alloc(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32);

    let cmd = device
        .allocate_command_buffers(VK_COMMAND_BUFFER_LEVEL_PRIMARY, 1)
        .unwrap();

    vkCmdBlock!{cmd[0],
        
    };

    let submit_info = VkSubmitInfoBuilder::new()
        .commandBufferCount(cmd.len() as u32)
        .pCommandBuffers(cmd.as_ptr())
        .waitSemaphoreCount(0)
        .build();

    let fence_info = VkFenceCreateInfoBuilder::new().flags(0).build();

    let fence = device.create_fence(fence_info, None).unwrap();
    device.queue_submit(0, &submit_info, 1, fence);

    // compute pipeline
    let mut pipeline = vk_instantiate!(VkPipeline);
    {
        let mut pipeline_cache = vk_instantiate!(VkPipelineCache);
        let pipeline_cache_create_info = VkPipelineCacheCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            initialDataSize: 0,
            pInitialData: null(),
        };
        unsafe {
            vk_assert(vkCreatePipelineCache(
                device.self_,
                &pipeline_cache_create_info,
                null(),
                &mut pipeline_cache,
            ));
        }

        let mut pipeline_layout = vk_instantiate!(VkPipelineLayout);
        let pipeline_layout_create_info = VkPipelineLayoutCreateInfoBuilder::new()
            .flags(0)
            .setLayoutCount(descriptor.set_layouts.len() as u32)
            .pSetLayouts(descriptor.set_layouts.as_ptr())
            .pushConstantRangeCount(0)
            .build();

        unsafe {
            vk_assert(vkCreatePipelineLayout(
                device.self_,
                &pipeline_layout_create_info,
                null(),
                &mut pipeline_layout,
            ));
        }

        let name = CString::new("main").unwrap();
        let ref_name = &name;

        // pipeline
        let pipeline_stage_create_info = VkPipelineShaderStageCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            stage: VK_SHADER_STAGE_COMPUTE_BIT,
            module: device.create_shader_module(COMP_SPV).unwrap(),
            pName: ref_name.as_ptr() as *const i8,
            pSpecializationInfo: null(),
        };

        println!("pipeline");
        let compute_pipeline_create_info = VkComputePipelineCreateInfoBuilder::new()
            .flags(0)
            .stage(pipeline_stage_create_info)
            .layout(pipeline_layout)
            .basePipelineIndex(0)    
            .build();

        unsafe {
            vk_assert(vkCreateComputePipelines(
                device.self_,
                pipeline_cache,
                1,
                &compute_pipeline_create_info,
                null(),
                &mut pipeline,
            ));
        }
    }

    // let vv = vec![1, 2];

    // commands
    // let cmd = device
    //     .allocate_command_buffer(VK_COMMAND_BUFFER_LEVEL_PRIMARY)
    //     .unwrap();
    {
        // let begin_info = VkCommandBufferBeginInfo {
        //     sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        //     pNext: null(),
        //     flags: 0,
        //     pInheritanceInfo: null(),
        // };
        // unsafe {
        //     vkBeginCommandBuffer(cmd, &begin_info);
        // }

        // let buf_barrier = VkBufferMemoryBarrier {
        //     sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
        //     pNext: null(),
        //     srcAccess: VK_ACCESS_TRANSFER_WRITE_BIT,
        //     dstAccessMask: VK_ACCESS_HOST_READ_BIT,
        //     srcQueueFamilyIndex: todo!(),
        //     dstQueueFamilyIndex: todo!(),
        //     buffer: buffer.,
        //     offset: todo!(),
        //     size: todo!(),
        // };
    }

    Ok(())
}
