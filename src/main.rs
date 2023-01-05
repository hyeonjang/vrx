use std::env;
use std::ffi::*;
use std::ptr::*;
use anyhow::*;
use vkcholesky::*;

const COMP_SPV: &[u8] = include_bytes!("./shader/cholesky.spv");

fn main() -> Result<()> {
    let vk_layer_path = env::var("VULKAN_SDK").unwrap();
    println!("{:?}", vk_layer_path);

    let device = Device::new();
    let descriptor = Descriptor::new(1, &device.self_);

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
        let pipeline_layout_create_info = VkPipelineLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: null(),
            flags: 0,
            setLayoutCount: descriptor.set_layouts.len() as u32,
            pSetLayouts: descriptor.set_layouts.as_ptr(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: null(),
        };
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
        let compute_pipeline_create_info = VkComputePipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            stage: pipeline_stage_create_info,
            layout: pipeline_layout,
            basePipelineHandle: null_mut(),
            basePipelineIndex: 0,
        };
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
        println!("here");
    }

    // commands
    let cmd = device.allocate_command_buffer(VK_COMMAND_BUFFER_LEVEL_PRIMARY).unwrap();
    {
        let begin_info = VkCommandBufferBeginInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: null(),
            flags: 0,
            pInheritanceInfo: null(),
        };
        unsafe {        
            vkBeginCommandBuffer(cmd, &begin_info);
        }

        let buf_barrier = VkBufferMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
            pNext: null(),
            srcAccessMask: todo!(),
            dstAccessMask: todo!(),
            srcQueueFamilyIndex: todo!(),
            dstQueueFamilyIndex: todo!(),
            buffer: todo!(),
            offset: todo!(),
            size: todo!(),
        };
    }

    Ok(())
}
