#![allow(dead_code)]

use std::ffi::CString;
use std::ops::Index;
use std::ptr::null;

use vrx::vx::*;
use vrx::*;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const R: usize, const C: usize> {
    values: [[T; R]; C],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(arrays: [[T; R]; C]) -> Self {
        Matrix { values: arrays }
    }

    pub fn len(&self) -> usize {
        R * C
    }

    pub fn shape(&self) -> (usize, usize) {
        (R, C)
    }
}

pub struct DynamicMatrix<T> {
    values: Vec<T>,
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0][index.1]
    }
}

pub trait MatrixSolver {}

pub enum SparesMatrixType {
    CSC = 0,
}

pub struct SparseMatrix<T, const U: usize> {
    values: Vec<T>,
    col_index: Vec<usize>,
    row_index: Vec<usize>,
}

pub type CSC<T> = SparseMatrix<T, 0>;
pub type CSR<T> = SparseMatrix<T, 1>;
pub type COO<T> = SparseMatrix<T, 2>;

pub trait SparseSolver {
    fn factorize();
}

const COMP_SPV: &[u8] = include_bytes!("./shader/cholesky.spv");
pub trait Factorizor {
    // fn LU(&self);
    fn cholesky(&self);
}

impl<T, const R: usize, const C: usize> Factorizor for Matrix<T, R, C>
where
    T: std::fmt::Debug + std::marker::Copy + Default,
{
    fn cholesky(&self) {
        let device = vx::Device::new(&[(vx::QueueType::computes, 1)]);
        println!("{:?}", device.queue_family_indices);

        let input_constant = PushConstant::new(
            VK_SHADER_STAGE_COMPUTE_BIT,
            self.values.as_ptr(),
            self.len() as u32,
        );

        let mut out_values = [[T::default(); R]; C];
        let shape = (out_values.len() as u32, out_values[0].len() as u32);
        let len = shape.0 * shape.1;
        let mut out_buffer = device
            .create_vxbuffer(
                &mut out_values[0][0],
                len as u32,
                VK_BUFFER_USAGE_STORAGE_BUFFER_BIT
                    | VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                    | VK_BUFFER_USAGE_TRANSFER_DST_BIT,
                0,
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .unwrap();
        out_buffer.map_to_gpu_and_unmap();

        let image_create_info = VkImageCreateInfoBuilder::new()
            .image_type(VK_IMAGE_TYPE_2D)
            .extent(VkExtent3D {
                width: shape.0 as u32,
                height: shape.1 as u32,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .format(VK_FORMAT_R32_SFLOAT)
            .tiling(VK_IMAGE_TILING_OPTIMAL)
            .initial_layout(VK_IMAGE_LAYOUT_UNDEFINED)
            .usage((VK_IMAGE_USAGE_SAMPLED_BIT | VK_IMAGE_USAGE_STORAGE_BIT) as u32)
            .samples(VK_SAMPLE_COUNT_1_BIT)
            .sharing_mode(VK_SHARING_MODE_EXCLUSIVE)
            .build();

        let out_image = device
            .create_vximage(image_create_info, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT)
            .unwrap();

        // let cmd = device
        //     .allocate_command_buffer(VK_COMMAND_BUFFER_LEVEL_PRIMARY)
        //     .unwrap();
        // vkCmdBlock! {
        //     THIS cmd;

        //     let copy = VkBufferImageCopy {
        //         bufferOffset: 0,
        //         bufferRowLength: 0,
        //         bufferImageHeight: 0,
        //         imageSubresource: VkImageSubresourceLayers { aspectMask:VK_IMAGE_ASPECT_COLOR_BIT as u32, mipLevel:0, baseArrayLayer:0, layerCount:1 },
        //         imageOffset: VkOffset3D { x: 0, y: 0, z: 0 },
        //         imageExtent: VkExtent3D { width: shape.0, height: shape.1, depth: 1 },
        //     };

        //     COPY_BUFFER_TO_IMAGE(
        //         *out_buffer.buffer(),
        //         *out_image.image(),
        //         VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        //         1,
        //         &copy
        //     );
        // };

        let spec0 = VkSpecializationMapEntry {
            constantID: 0,
            offset: 0,
            size: std::mem::size_of::<u32>(),
        };

        let spec1 = VkSpecializationMapEntry {
            constantID: 1,
            offset: 0,
            size: std::mem::size_of::<u32>(),
        };

        let map_entries = [spec0, spec1];
        let spec_info = VkSpecializationInfo {
            mapEntryCount: 2,
            pMapEntries: map_entries.as_ptr(),
            dataSize: std::mem::size_of::<u32>() * 2,
            pData: [R as u32, C as u32].as_ptr() as *const std::ffi::c_void,
        };

        //
        // construct compute pipeline
        //
        let buffer_descriptor = VkDescriptorBufferInfo {
            buffer: *out_buffer.buffer(),
            offset: 0,
            range: out_buffer.vksize(),
        };

        let sampler_create_info = VkSamplerCreateInfoBuilder::new().build();

        let sampler = device.create_sampler(&sampler_create_info, None).unwrap();

        let image_view_create_info = VkImageViewCreateInfoBuilder::new()
            .image(*out_image.image())
            .view_type(VK_IMAGE_VIEW_TYPE_2D)
            .format(VK_FORMAT_R32_SFLOAT)
            .subresource_range(VkImageSubresourceRange {
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT as u32,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            })
            .build();

        let image_view = device
            .create_image_view(&image_view_create_info, None)
            .unwrap();

        let image_descriptor = VkDescriptorImageInfo {
            sampler: sampler,
            imageView: image_view,
            imageLayout: 0,
        };

        let layout_bindings = vec![
            VkDescriptorSetLayoutBinding {
                binding: 0,
                descriptorType: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
                descriptorCount: 1,
                stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as u32,
                pImmutableSamplers: null(),
            },
            VkDescriptorSetLayoutBinding {
                binding: 1,
                descriptorType: VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
                descriptorCount: 1,
                stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as u32,
                pImmutableSamplers: null(),
            },
        ];

        let pool_sizes = vec![
            VkDescriptorPoolSize {
                type_: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
                descriptorCount: 1,
            },
            VkDescriptorPoolSize {
                type_: VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
                descriptorCount: 1,
            },
        ];

        let descriptor_pool_create_info = VkDescriptorPoolCreateInfoBuilder::new()
            .max_sets(2)
            .pool_size_count(pool_sizes.len() as u32)
            .p_pool_sizes(pool_sizes.as_ptr())
            .build();

        let descriptor_pool = device
            .create_descriptor_pool(&descriptor_pool_create_info, None)
            .unwrap();

        let descriptor_set_layout_create_info = VkDescriptorSetLayoutCreateInfoBuilder::new()
            .binding_count(layout_bindings.len() as u32)
            .p_bindings(layout_bindings.as_ptr())
            .build();

        let descriptor_set_layout = device
            .create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
            .unwrap();

        let descriptor_set_alloc_info = VkDescriptorSetAllocateInfoBuilder::new()
            .descriptor_pool(descriptor_pool)
            .descriptor_set_count(1)
            .p_set_layouts(&descriptor_set_layout)
            .build();

        let descriptor_sets = device
            .allocate_descriptor_sets(&descriptor_set_alloc_info)
            .unwrap();

        let write_desc_sets = vec![
            VkWriteDescriptorSetBuilder::new()
                .dst_set(descriptor_sets[0])
                .dst_binding(0)
                .descriptor_type(VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
                .descriptor_count(1)
                .p_buffer_info(&buffer_descriptor)
                .build(),
            // VkWriteDescriptorSetBuilder::new()
            //     .dstSet(descriptor_sets[1])
            //     .descriptorType(VK_DESCRIPTOR_TYPE_STORAGE_IMAGE)
            //     .descriptorCount(1)
            //     .pImageInfo(&image_descriptor)
            //     .build(),
        ];
        device.update_descriptor_sets(write_desc_sets.len(), write_desc_sets.as_ptr());

        // compute pipeline
        let name = CString::new("main").unwrap();
        let ref_name = &name;

        // pipeline
        let pipeline_cache_create_info = VkPipelineCacheCreateInfoBuilder::new()
            .flags(0)
            .initial_data_size(0)
            .build();
        let pipeline_cache = device
            .create_pipeline_cache(&pipeline_cache_create_info)
            .unwrap();

        let pipeline_layout_create_info = VkPipelineLayoutCreateInfoBuilder::new()
            .flags(0)
            .push_constant_range_count(1)
            .p_push_constant_ranges(&input_constant.range())
            .set_layout_count(1)
            .p_set_layouts(&descriptor_set_layout)
            .build();

        let pipeline_layout = device
            .create_pipeline_layout(&pipeline_layout_create_info)
            .unwrap();

        let pipeline_stage_create_info = VkPipelineShaderStageCreateInfoBuilder::new()
            .stage(VK_SHADER_STAGE_COMPUTE_BIT)
            .module(device.create_shader_module(COMP_SPV).unwrap())
            .p_name(ref_name.as_ptr() as *const i8)
            .p_specialization_info(&spec_info)
            .build();

        let compute_pipeline_create_info = VkComputePipelineCreateInfoBuilder::new()
            .flags(0)
            .stage(pipeline_stage_create_info)
            .layout(pipeline_layout)
            .base_pipeline_index(0)
            .build();
        let pipelines = device
            .create_compute_pipelines(pipeline_cache, 1, &compute_pipeline_create_info)
            .unwrap();
        let pipeline = pipelines[0];

        //
        // pipepline submit commands
        let cmd = device
            .allocate_command_buffer(vx::QueueType::computes, VK_COMMAND_BUFFER_LEVEL_PRIMARY)
            .unwrap();
        vkCmdBlock! {
            THIS cmd;

            BIND_PIPELINE(
                VK_PIPELINE_BIND_POINT_COMPUTE,
                pipeline
            );

            PUSH_CONSTANT(
                pipeline_layout,
                input_constant.stage(),
                0,
                input_constant.vksize(),
                input_constant.as_ptr()
            );

            BIND_DESCRIPTOR_SETS(
                VK_PIPELINE_BIND_POINT_COMPUTE,
                pipeline_layout,
                0, 1,
                descriptor_sets.as_ptr(),
                0, null()
            );

            DISPATCH(32, 1, 1);

            let buffer_barrier = VkBufferMemoryBarrierBuilder::new()
                .buffer(*(out_buffer.buffer()))
                .size(VK_WHOLE_SIZE as u64)
                .src_access_mask(VK_ACCESS_SHADER_WRITE_BIT.try_into().unwrap())
                .dst_access_mask(VK_ACCESS_TRANSFER_READ_BIT.try_into().unwrap())
                .src_queue_family_index(VK_QUEUE_FAMILY_IGNORED as u32)
                .dst_queue_family_index(VK_QUEUE_FAMILY_IGNORED as u32)
                .build();

            PIPELINE_BARRIER(
                VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT,
                VK_PIPELINE_STAGE_TRANSFER_BIT,
                0, 0, null(), 1,
                &buffer_barrier, 0, null()
            );
        };

        let fence_create_info = VkFenceCreateInfoBuilder::new()
            .flags(VK_FENCE_CREATE_SIGNALED_BIT.try_into().unwrap())
            .build();
        let fence1 = device.create_fence(&fence_create_info, None).unwrap();
        device.reset_fence(1, &fence1);

        let wait_stage_mask = VK_PIPELINE_STAGE_TRANSFER_BIT as u32;
        let submit_info = VkSubmitInfoBuilder::new()
            .p_wait_dst_stage_mask(&wait_stage_mask)
            .command_buffer_count(1)
            .p_command_buffers(&cmd)
            .build();

        device.queue_submit(vx::QueueType::computes, 0, 1, &submit_info, fence1);
        device.wait_for_fence(1, &fence1, false, u64::MAX);

        // let new_mapped = out_buffer
        //     .map_memory(0, VK_WHOLE_SIZE as u64, 0)
        //     .unwrap();
        // let mapped_ranges = VkMappedMemoryRangeBuilder::new()
        //     .memory(*out_buffer.memory())
        //     .offset(0)
        //     .size(out_buffer.vksize())
        //     .build();
        //     out_buffer.invalidate_mapped_memory_ranges(1, &mapped_ranges);

        // let mut finalle = [[T::default(); R]; C];
        // println!("finalle {:?}", finalle);
        // unsafe {
        //     memcpy(new_mapped.cast(), finalle.as_mut_ptr(), 2);
        // }
        // println!("finalle {:?}", finalle);
        // out_buffer.unmap_memory();

        let mapped = out_buffer.map_to_cpu_and_unmap();
        println!("{:?}", mapped);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        println!("some");
    }
}
