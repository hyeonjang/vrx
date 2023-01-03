use vkcholesky::*;
use std::ptr::*;
use std::env;

fn main() {
    let vk_layer_path = env::var("VULKAN_SDK").unwrap();
    println!("{:?}", vk_layer_path);

    let device = Device::new();
    println!("device");
    println!("descriptor");
    
    let descriptor = Descriptor::new(1, &device.self_);

    println!("device");
    // compute pipeline
    unsafe {
        let mut pipeline_cache = vk_instantiate!(VkPipelineCache);
        let pipeline_cache_create_info = VkPipelineCacheCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            initialDataSize: 0,
            pInitialData: null(),
        };
        vk_assert(vkCreatePipelineCache(device.self_, &pipeline_cache_create_info, null(), &mut pipeline_cache));

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
        vk_assert(vkCreatePipelineLayout(device.self_, &pipeline_layout_create_info, null(), &mut pipeline_layout));

        // pipeline
        let mut pipeline = vk_instantiate!(VkPipeline);
        let pipeline_stage_create_info = VkPipelineShaderStageCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            stage: todo!(),
            module: todo!(),
            pName: todo!(),
            pSpecializationInfo: todo!(),
        };

        let compute_pipeline_create_info = VkComputePipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            stage: todo!(),
            layout: pipeline_layout,
            basePipelineHandle: todo!(),
            basePipelineIndex: todo!(),
        };
        // vk_assert(vkCreateComputePipelines(device));
    }
}
