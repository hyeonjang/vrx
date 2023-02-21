use std::ffi::CString;
use std::ops::Index;
use std::ptr::null;

use vkcholesky::vx::*;
use vkcholesky::*;

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
    fn cholesky(&self);
}

impl<T, const R: usize, const C: usize> Factorizor for Matrix<T, R, C>
where
    T: std::fmt::Debug + std::marker::Copy + Default,
{
    fn cholesky(&self) {
        let device = vx::Device::new();

        let input_constant = PushConstant::new(
            VK_SHADER_STAGE_COMPUTE_BIT,
            self.values.as_ptr(),
            self.len() as u32,
        );

        let mut out_values = [[T::default(); R]; C];
        let len = out_values.len() * out_values[0].len();
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
        let descriptor = device.create_descriptor(1).unwrap();
        let buffer_descriptor = VkDescriptorBufferInfo {
            buffer: *out_buffer.buffer(),
            offset: 0,
            range: out_buffer.vksize(),
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
        let name = CString::new("main").unwrap();
        let ref_name = &name;

        // pipeline
        let pipeline_cache_create_info = VkPipelineCacheCreateInfoBuilder::new()
            .flags(0)
            .initialDataSize(0)
            .build();
        let pipeline_cache = device
            .create_pipeline_cache(&pipeline_cache_create_info)
            .unwrap();

        let pipeline_layout_create_info = VkPipelineLayoutCreateInfoBuilder::new()
            .flags(0)
            .pushConstantRangeCount(1)
            .pPushConstantRanges(&input_constant.range())
            .setLayoutCount(descriptor.set_layouts.len() as u32)
            .pSetLayouts(descriptor.set_layouts.as_ptr())
            .build();

        let pipeline_layout = device
            .create_pipeline_layout(&pipeline_layout_create_info)
            .unwrap();

        let pipeline_stage_create_info = VkPipelineShaderStageCreateInfoBuilder::new()
            .stage(VK_SHADER_STAGE_COMPUTE_BIT)
            .module(device.create_shader_module(COMP_SPV).unwrap())
            .pName(ref_name.as_ptr() as *const i8)
            .pSpecializationInfo(&spec_info)
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
                descriptor.sets.as_ptr(),
                0, null()
            );

            DISPATCH(32, 1, 1);

            let buffer_barrier = VkBufferMemoryBarrierBuilder::new()
                .buffer(*(out_buffer.buffer()))
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
                &buffer_barrier, 0, null()
            );
        };

        let fence_create_info = VkFenceCreateInfoBuilder::new()
            .flags(VK_FENCE_CREATE_SIGNALED_BIT.try_into().unwrap())
            .build();
        let fence1 = device.create_fence(fence_create_info, None).unwrap();
        device.reset_fence(1, &fence1);

        let wait_stage_mask = VK_PIPELINE_STAGE_TRANSFER_BIT as u32;
        let submit_info = VkSubmitInfoBuilder::new()
            .pWaitDstStageMask(&wait_stage_mask)
            .commandBufferCount(1)
            .pCommandBuffers(&cmd)
            .build();

        device.queue_submit(0, &submit_info, 1, fence1);
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
