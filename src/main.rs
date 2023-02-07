use anyhow::*;
use std::ffi::*;
use std::ptr::*;
use vkcholesky::*;

const COMP_SPV: &[u8] = include_bytes!("./shader/cholesky.spv");

fn main() -> Result<()> {
    let device = Device::new();

    let host_data: Vec<i32> = (0..32).collect();
    let device_data = vec![0; 32];
    let mut host_buffer = device
        .create_buffer(
            host_data.clone(),
            VK_BUFFER_USAGE_TRANSFER_SRC_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT,
            0,
        )
        .unwrap();
    host_buffer.alloc(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT);
    let mut host_mapped = host_buffer.map_memory(0, host_buffer.vksize(), 0).unwrap();
    host_mapped.copy_from_slice(host_data.clone().as_slice());
    host_buffer.unmap_memory();
    host_buffer.bind_buffer_memory(0);
    println!("{:?}", host_mapped);

    host_buffer.map_memory(0, host_buffer.vksize(), 0);
    let mapped_range = VkMappedMemoryRangeBuilder::new()
        .memory(*host_buffer.memory())
        .offset(0)
        .size(VK_WHOLE_SIZE as u64)
        .build();
    host_buffer.flush_mapped_memory_range(1, &mapped_range);
    host_buffer.unmap_memory();

    let mut device_buffer = device
        .create_buffer(
            device_data,
            VK_BUFFER_USAGE_STORAGE_BUFFER_BIT
                | VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                | VK_BUFFER_USAGE_TRANSFER_DST_BIT,
            0,
        )
        .unwrap();
    device_buffer.alloc(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    device_buffer.bind_buffer_memory(0);

    // commands
    let cmd_copy = device
        .allocate_command_buffer(VK_COMMAND_BUFFER_LEVEL_PRIMARY)
        .unwrap();
    vkCmdBlock! {
        THIS cmd_copy;

        let buffer_copy = VkBufferCopy { srcOffset: 0, dstOffset: 0, size: host_buffer.vksize() };
        COPY_BUFFER(host_buffer.as_raw(), device_buffer.as_raw(), 1, &buffer_copy);
    }

    let submit_info = VkSubmitInfoBuilder::new()
        .commandBufferCount(1 as u32)
        .pCommandBuffers(&cmd_copy)
        .waitSemaphoreCount(0)
        .build();

    let fence_create_info = VkFenceCreateInfoBuilder::new().flags(0).build();
    let fence = device.create_fence(fence_create_info, None).unwrap();
    device.queue_submit(0, &submit_info, 1, fence);
    device.wait_for_fence(1, &fence, true, u64::MAX);
    device.destroy_fence(fence, None);
    device.free_commands_buffers(1, &cmd_copy);

    let device_mapped = device_buffer
        .map_memory(0, device_buffer.vksize(), 0)
        .unwrap();
    let mut mapped = vec![12; 32];

    println!("debug {:?}", device_mapped.as_ptr());
    println!("debug {:?}", device_mapped);

    mapped.copy_from_slice(device_mapped.as_slice());
    device_buffer.unmap_memory();

    println!("{:?}", device_buffer.data);
    println!("{:?}", device_mapped);
    //
    // construct compute pipeline
    //
    let descriptor = device.create_descriptor(1).unwrap();
    let buffer_descriptor = VkDescriptorBufferInfo {
        buffer: device_buffer.as_raw(),
        offset: 0,
        range: device_buffer.vksize(),
    };

    let write_desc_set = VkWriteDescriptorSetBuilder::new()
        .dstSet(descriptor.sets[0])
        .dstBinding(0)
        .descriptorType(VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .descriptorCount(1)
        .pBufferInfo(&buffer_descriptor)
        .build();
    descriptor.update(vec![write_desc_set]);

    // compute pipeline
    let pipeline_cache_create_info = VkPipelineCacheCreateInfoBuilder::new()
        .flags(0)
        .initialDataSize(0)
        .build();
    let pipeline_cache = device
        .create_pipeline_cache(&pipeline_cache_create_info)
        .unwrap();

    let pipeline_layout_create_info = VkPipelineLayoutCreateInfoBuilder::new()
        .flags(0)
        .setLayoutCount(descriptor.set_layouts.len() as u32)
        .pSetLayouts(descriptor.set_layouts.as_ptr())
        .pushConstantRangeCount(0)
        .build();
    let pipeline_layout = device
        .create_pipeline_layout(&pipeline_layout_create_info)
        .unwrap();

    let name = CString::new("main").unwrap();
    let ref_name = &name;

    // pipeline
    let pipeline_stage_create_info = VkPipelineShaderStageCreateInfoBuilder::new()
        .stage(VK_SHADER_STAGE_COMPUTE_BIT)
        .module(device.create_shader_module(COMP_SPV).unwrap())
        .pName(ref_name.as_ptr() as *const i8)
        .build();
    let compute_pipeline_create_info = VkComputePipelineCreateInfoBuilder::new()
        .flags(0)
        .stage(pipeline_stage_create_info)
        .layout(pipeline_layout)
        .basePipelineIndex(0)
        .build();
    let pipelines = device
        .create_compute_pipelines(pipeline_cache, 1, &compute_pipeline_create_info)
        .unwrap();
    let pipeline = pipelines[0];

    //
    // pipepline submit commands
    let cmd = device
        .allocate_command_buffer(VK_COMMAND_BUFFER_LEVEL_PRIMARY)
        .unwrap();
    vkCmdBlock! {
        THIS cmd;

        let buffer_barrier0 = VkBufferMemoryBarrierBuilder::new()
            .buffer(device_buffer.as_raw())
            .size(VK_WHOLE_SIZE as u64) //?? bug? VK_WHOLE_SIZE CHECKING needed
            .srcAccessMask(VK_ACCESS_HOST_WRITE_BIT.try_into().unwrap())
            .dstAccessMask(VK_ACCESS_SHADER_READ_BIT.try_into().unwrap())
            .srcQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED as u32)
            .dstQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED as u32)
            .build();

        PIPELINE_BARRIER(
            VK_PIPELINE_STAGE_HOST_BIT,
            VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT,
            0, 0, null(), 1,
            &buffer_barrier0, 0, null()
        );

        BIND_PIPELINE(
            VK_PIPELINE_BIND_POINT_COMPUTE,
            pipeline
        );

        BIND_DESCRIPTOR_SETS(
            VK_PIPELINE_BIND_POINT_COMPUTE,
            pipeline_layout,
            0, 1,
            descriptor.sets.as_ptr(),
            0, null()
        );

        DISPATCH(32, 1, 1);

        let buffer_barrier1 = VkBufferMemoryBarrierBuilder::new()
            .buffer(device_buffer.as_raw())
            .size(VK_WHOLE_SIZE as u64)
            .srcAccessMask(VK_ACCESS_SHADER_WRITE_BIT.try_into().unwrap())
            .dstAccessMask(VK_ACCESS_TRANSFER_READ_BIT.try_into().unwrap())
            .srcQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED as u32)
            .dstQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED as u32)
            .build();

        PIPELINE_BARRIER(
            VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT,
            VK_PIPELINE_STAGE_TRANSFER_BIT,
            0, 0, null(), 1,
            &buffer_barrier1, 0, null()
        );

        // let buffer_copy = VkBufferCopy { srcOffset: 0, dstOffset: 0, size: device_buffer.vksize() };
        // COPY_BUFFER(device_buffer.as_raw(), host_buffer.as_raw(), 1, &buffer_copy);

        // let buffer_barrier2 = VkBufferMemoryBarrierBuilder::new()
        //     .buffer(host_buffer.as_raw())
        //     .size(VK_WHOLE_SIZE as u64)
        //     .srcAccessMask(VK_ACCESS_TRANSFER_WRITE_BIT.try_into().unwrap())
        //     .dstAccessMask(VK_ACCESS_HOST_READ_BIT.try_into().unwrap())
        //     .srcQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED as u32)
        //     .dstQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED as u32)
        //     .build();

        // PIPELINE_BARRIER(
        //     VK_PIPELINE_STAGE_TRANSFER_BIT,
        //     VK_PIPELINE_STAGE_HOST_BIT,
        //     0, 0, null(), 1,
        //     &buffer_barrier2, 0, null()
        // );
    };
    let fence_create_info = VkFenceCreateInfoBuilder::new()
        .flags(VK_FENCE_CREATE_SIGNALED_BIT.try_into().unwrap())
        .build();
    let fence = device.create_fence(fence_create_info, None).unwrap();
    device.reset_fence(1, &fence);

    let wait_stage_mask = VK_PIPELINE_STAGE_TRANSFER_BIT as u32;
    let submit_info = VkSubmitInfoBuilder::new()
        .pWaitDstStageMask(&wait_stage_mask)
        .commandBufferCount(1)
        .pCommandBuffers(&cmd)
        .build();

    device.queue_submit(0, &submit_info, 1, fence);
    device.wait_for_fence(1, &fence, true, u64::MAX);

    let new_mapped = host_buffer.map_memory(0, VK_WHOLE_SIZE as u64, 0).unwrap();
    // let mapped_ranges = VkMappedMemoryRangeBuilder::new()
    // .memory(host_buffer.memory)
    //     .offset(0)
    //     .size(host_buffer.vksize())
    //     .build();
    // host_buffer.invalidate_mapped_memory_ranges(1, &mapped_ranges);

    let mut finalle = vec![2021; 32];
    println!("finalle {:?}", finalle);
    finalle.copy_from_slice(new_mapped.as_slice());
    println!("finalle {:?}", finalle);
    host_buffer.unmap_memory();

    host_buffer.free_memory(None);
    host_buffer.destroy(None);
    Ok(())
}
