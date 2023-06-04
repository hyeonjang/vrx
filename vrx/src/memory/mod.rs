#[macro_use]
use crate::*;

use func_static::vk_instantiate;

pub struct Descriptor<'a> {
    pub set_layouts: Option<Vec<VkDescriptorSetLayout>>,   // later initialize
    pub sets: Option<Vec<VkDescriptorSet>>,                 // later initialize
    pub map: Option<std::collections::HashMap<u32, u32>>,  // later initialize

    pub pool: VkDescriptorPool,
    device: &'a VkDevice,
}

impl<'a> Descriptor<'a> {
    pub fn new(descriptor_pool_size: &[VkDescriptorPoolSize], device: &'a VkDevice) -> Self {

        let max_sets = descriptor_pool_size.iter().fold(0, |init, pool_size| init + pool_size.descriptorCount);

        let desc_pool_create_info = VkDescriptorPoolCreateInfoBuilder::new()
            .pool_size_count(descriptor_pool_size.len() as u32)
            .p_pool_sizes(descriptor_pool_size.as_ptr())
            .max_sets(max_sets)
            .build();

        Self {
            pool: device.create_descriptor_pool(&desc_pool_create_info, None),
            set_layouts: None,
            sets: None,
            map: None,
            device: device,
        }
    }

    pub fn create_set_layouts(&mut self, desc_layout_create_infos: &[VkDescriptorSetLayoutCreateInfo]) {
        self.set_layouts = Some(desc_layout_create_infos.iter().map( |&info| self.device.create_descriptor_set_layout(&info, None)).collect());
    }

    pub fn allocate_sets(&mut self, set_count: u32) {

        let allocate_info = VkDescriptorSetAllocateInfoBuilder::new()
            .descriptor_pool(self.pool)
            .descriptor_set_count(set_count)
            .p_set_layouts(self.set_layouts.as_ref().unwrap().as_ptr())
            .build();

        self.sets = Some(self.device.allocate_descriptor_sets(&allocate_info));
    }

    pub fn update(&self, desc_writes: &[VkWriteDescriptorSet]) {
        self.device.update_descriptor_sets(desc_writes, &[]);
    }

    pub fn copy(&self, desc_copies: &[VkCopyDescriptorSet]) {
        self.device.update_descriptor_sets(&[], desc_copies);
    }

    pub fn update_and_copy(&self, desc_writes: &[VkWriteDescriptorSet], desc_copies: &[VkCopyDescriptorSet]) {
        self.device.update_descriptor_sets(desc_writes, desc_copies);
    }
}

pub trait MemoryFunctions {
    // trait getter
    fn device(&self) -> &VkDevice;
    fn buffer(&self) -> Option<&VkBuffer> {
        None
    }
    fn buffer_mut(&mut self) -> Option<&mut VkBuffer> {
        None
    }
    fn image(&self) -> Option<&VkImage> {
        None
    }
    fn image_mut(&mut self) -> Option<&mut VkImage> {
        None
    }
    fn memory(&self) -> &VkDeviceMemory;
    fn memory_mut(&mut self) -> &mut VkDeviceMemory;

    /// functional
    fn get_memory_requirements(&self) -> VkMemoryRequirements;
    fn allocate_memory(&mut self, mem_prop_flags: VkMemoryPropertyFlagBits);

    fn map_memory(&self, offset: u64, size: u64, flags: u32) -> anyhow::Result<*mut std::os::raw::c_void> {
        self.device().map_memory(offset, size, flags, self.memory())
    }

    fn unmap_memory(&self) {
        self.device().unmap_memory(self.memory());
    }

    fn free_memory(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        self.device().free_memory(self.memory(), p_allocator);
    }

    fn flush_mapped_memory_range(
        &self,
        memory_range_count: u32,
        p_memory_ranges: *const VkMappedMemoryRange,
    ) {
        unsafe {
            vkFlushMappedMemoryRanges(*self.device(), memory_range_count, p_memory_ranges);
        }
    }

    fn invalidate_mapped_memory_ranges(
        &self,
        memory_range_count: u32,
        p_memory_ranges: *const VkMappedMemoryRange,
    ) {
        unsafe {
            vkInvalidateMappedMemoryRanges(*self.device(), memory_range_count, p_memory_ranges);
        }
    }
    fn bind_buffer_memory(&self, offset: VkDeviceSize) {}
    fn bind_image_memory(&self, offset: VkDeviceSize) {}
}

#[derive(Debug)]
pub struct VxBuffer<'a, T> {
    device: &'a VkDevice,

    pub buffer: VkBuffer,
    memory: VkDeviceMemory,

    data: Option<*const T>,
    len: u32,
}

impl<'a, T> MemoryFunctions for VxBuffer<'a, T> {
    fn device(&self) -> &VkDevice {
        self.device
    }

    fn buffer(&self) -> Option<&VkBuffer> {
        Some(&self.buffer)
    }

    fn buffer_mut(&mut self) -> Option<&mut VkBuffer> {
        Some(&mut self.buffer)
    }

    fn memory(&self) -> &VkDeviceMemory {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut VkDeviceMemory {
        &mut self.memory
    }

    fn get_memory_requirements(&self) -> VkMemoryRequirements {
        self.device.get_buffer_memory_requirements(self.buffer)
    }

    fn allocate_memory(&mut self, mem_prop_flags: VkMemoryPropertyFlagBits) {
        let ctx = vulkan_context();

        //@@ to static
        let mut mem_prop = ctx.get_physical_device_memory_properties();
        let mut collect: Vec<u32> = (0..mem_prop.memoryTypeCount).collect();
        collect.retain(|i| {
            mem_prop.memoryTypes[*i as usize].propertyFlags
                & mem_prop_flags as VkMemoryPropertyFlags
                == mem_prop_flags as VkMemoryPropertyFlags
        });

        let mut mem_req = self.get_memory_requirements();
        let mut mem_alloc_info = VkMemoryAllocateInfoBuilder::new()
            .allocation_size(mem_req.size)
            .memory_type_index(collect[0])
            .build();

        self.memory = self.device().allocate_memory(&mem_alloc_info, None);
    }

    fn bind_buffer_memory(&self, offset: VkDeviceSize) {
        self.device().bind_buffer_memory(self.buffer, self.memory, offset);
    }
}

impl<'a, T> Drop for VxBuffer<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.destroy(None);
        }
    }
}

impl<'a, T> VxBuffer<'a, T> {
    pub fn new(
        data: Option<*const T>,
        len: u32,
        flags: VkBufferCreateFlags,
        usage: VkBufferUsageFlags,
        mem_prop_flags: VkMemoryPropertyFlagBits,
        device: &'a VkDevice,
    ) -> Self {
        let mem = vk_instantiate!(VkDeviceMemory);

        let info = VkBufferCreateInfoBuilder::new()
            .flags(flags)
            .size((len * std::mem::size_of::<T>() as u32) as u64)
            .usage(usage)
            .sharing_mode(VK_SHARING_MODE_EXCLUSIVE)
            .build();
        let buf = device.create_buffer(&info, None);

        let mut buffer = Self {
            buffer: buf,
            memory: mem,
            data: data,
            len: len,
            device: device,
        };

        buffer.allocate_memory(mem_prop_flags);
        buffer.bind_buffer_memory(0);

        buffer
    }

    pub fn buffer(&self) -> &VkBuffer {
        &self.buffer
    }

    pub fn destroy(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        self.device.destroy_buffer(self.buffer, p_allocator);
        self.device.free_memory(self.memory(), p_allocator);
    }

    // mappings
    pub fn map(&mut self, len: usize, data: *const T) {
        let mapped = self.map_memory(0, (std::mem::size_of::<T>() * len) as u64, 0).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(data, mapped.cast(), len);
        }
        self.unmap_memory();
    }

    pub fn map_to_gpu_and_unmap(&self) {
        let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(self.data.unwrap(), mapped.cast(), self.len as usize);
        }
        self.unmap_memory();
    }

    pub fn map_to_cpu_and_unmap(&mut self) -> Vec<T>
    where
        T: std::clone::Clone + Default,
    {
        let mut output = vec![T::default(); self.len as usize];

        let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(mapped.cast(), output.as_mut_ptr(), self.len as usize);
        }
        self.unmap_memory();

        output
    }

    pub fn vksize(&self) -> VkDeviceSize {
        (self.len * std::mem::size_of::<T>() as u32) as VkDeviceSize
    }
}

pub trait TextureOperator {

}

pub struct Texture<'a> {
    image: VkImage,             // memory layout
    memory: VkDeviceMemory,     // real memory

    device: &'a VkDevice,
}

impl<'a> Texture<'a> {
    pub fn new(
        mem_prop_flags: VkMemoryPropertyFlagBits,
        device: &'a VkDevice,
    ) -> Self {
        let mut texture = Self {
            image: std::ptr::null_mut(),
            memory: std::ptr::null_mut(),

            device: device,
        };

        // texture.allocate_memory(mem_prop_flags);
        texture
    }

    pub fn image(&self) -> &VkImage {
        &self.image
    }

    pub fn make_image(&mut self, image_create_info: &VkImageCreateInfo) {
        self.image = self.device.create_image(image_create_info, None);
    }
}

impl<'a> MemoryFunctions for Texture<'a> {
    fn device(&self) -> &VkDevice {
        self.device
    }

    fn image(&self) -> Option<&VkImage> {
        Some(&self.image)
    }

    fn image_mut(&mut self) -> Option<&mut VkImage> {
        Some(&mut self.image)
    }

    fn memory(&self) -> &VkDeviceMemory {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut VkDeviceMemory {
        &mut self.memory
    }

    fn get_memory_requirements(&self) -> VkMemoryRequirements {
        unsafe {
            let mut mem_req = VkMemoryRequirements {
                size: 0,
                alignment: 0,
                memoryTypeBits: 0,
            };
            vkGetImageMemoryRequirements(*self.device(), *self.image(), &mut mem_req);

            mem_req
        }
    }

    fn allocate_memory(&mut self, mem_prop_flags: VkMemoryPropertyFlagBits) {
        let ctx = vulkan_context();

        let mut mem_prop = ctx.get_physical_device_memory_properties();
        let mut collect: Vec<u32> = (0..mem_prop.memoryTypeCount).collect();
        collect.retain(|i| {
            mem_prop.memoryTypes[*i as usize].propertyFlags
                & mem_prop_flags as VkMemoryPropertyFlags
                == mem_prop_flags as VkMemoryPropertyFlags
        });

        let mut mem_req = self.get_memory_requirements();
        let mut mem_alloc_info = VkMemoryAllocateInfoBuilder::new()
            .allocation_size(mem_req.size)
            .memory_type_index(collect[0])
            .build();

        self.memory = self.device().allocate_memory(&mem_alloc_info, None);
    }

    fn bind_buffer_memory(&self, offset: VkDeviceSize) {
        unimplemented!();
    }
    fn bind_image_memory(&self, offset: VkDeviceSize) {
        unsafe {
            vk_assert(vkBindImageMemory(
                *self.device,
                self.image,
                self.memory,
                offset,
            ));
        }
    }
}

pub struct PushConstant<T> {
    stage: VkShaderStageFlags,
    data: *const T,
    size: u32,
}

impl<T> PushConstant<T> {
    pub fn new(stage: VkShaderStageFlagBits, data: *const T, size: u32) -> Self {
        Self {
            stage: stage as u32,
            data: data,
            size: size,
        }
    }

    pub fn vksize(&self) -> u32 {
        (self.size * std::mem::size_of::<T>() as u32) as u32
    }

    pub fn as_ptr(&self) -> *const std::os::raw::c_void {
        self.data as *const std::os::raw::c_void
    }

    pub fn stage(&self) -> VkShaderStageFlags {
        self.stage
    }

    pub fn range(&self) -> VkPushConstantRange {
        VkPushConstantRange {
            stageFlags: self.stage,
            offset: 0,
            size: self.vksize(),
        }
    }

    pub fn range_custom(&self, offset: u32, size: u32) -> VkPushConstantRange {
        VkPushConstantRange {
            stageFlags: self.stage,
            offset: offset,
            size: size,
        }
    }
}
