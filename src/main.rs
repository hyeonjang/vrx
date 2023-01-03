use vkcholesky::*;
use std::ptr::*;

fn main() {

    let device = Device::new();
    println!("device");
    
    let descriptor = Descriptor::new(0, &device.self_);

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
            setLayoutCount: descriptor.count,
            pSetLayouts: descriptor.set_layouts.as_ptr(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: null(),
        };
        vk_assert(vkCreatePipelineLayout(device.self_, &pipeline_layout_create_info, null(), &mut pipeline_layout));

        // let mut pipeline = vk_instantiate!(VkPipeline);
        // let compute_pipeline_create_info = VkComputePipelineCreateInfo {
        //     sType: todo!(),
        //     pNext: todo!(),
        //     flags: todo!(),
        //     stage: todo!(),
        //     layout: todo!(),
        //     basePipelineHandle: todo!(),
        //     basePipelineIndex: todo!(),
        // };
        // vk_assert(vkCreateComputePipelines(device));
    }
}
