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

mod memory_function {

    // use crate::{VkDevice, VkBuffer, VkImage, VkDeviceMemory, VkMemoryAllocateInfoBuilder, VkMemoryPropertyFlags } ;
    use crate::*;

    /// functional
    fn get_phyiscal_device_memory_property_collect(mem_prop_flags: VkMemoryPropertyFlags) -> Vec<u32>{
        let ctx = vulkan_context();

        //@@ to static
        let mut mem_prop = ctx.get_physical_device_memory_properties();
        let mut collect: Vec<u32> = (0..mem_prop.memoryTypeCount).collect();
        collect.retain(|i| {
            mem_prop.memoryTypes[*i as usize].propertyFlags
                & mem_prop_flags
                == mem_prop_flags
        });
        collect
    }

    pub fn allocate_buffer_memory(device: &VkDevice, buffer: VkBuffer, mem_prop_flags: VkMemoryPropertyFlags) -> VkDeviceMemory {

        let collect = get_phyiscal_device_memory_property_collect(mem_prop_flags);

        let mut mem_req = device.get_buffer_memory_requirements(buffer);
        let mut mem_alloc_info = VkMemoryAllocateInfoBuilder::new()
            .allocation_size(mem_req.size)
            .memory_type_index(collect[0])
            .build();

        device.allocate_memory(&mem_alloc_info, None)
    }

    pub fn allocate_image_memory(device: &VkDevice, image: VkImage, mem_prop_flags: VkMemoryPropertyFlags) -> VkDeviceMemory {

        let collect =get_phyiscal_device_memory_property_collect(mem_prop_flags);

        let mut mem_req = device.get_image_memory_requirements(image);
        let mut mem_alloc_info = VkMemoryAllocateInfoBuilder::new()
            .allocation_size(mem_req.size)
            .memory_type_index(collect[0])
            .build();

        device.allocate_memory(&mem_alloc_info, None)
    }
}

pub trait MemoryFunctions {

    fn device(&self) -> &VkDevice;
    fn memory(&self) -> &VkDeviceMemory;

    fn map_memory(&self, offset: u64, size: u64, flags: u32) -> anyhow::Result<*mut std::os::raw::c_void> {
        self.device().map_memory(offset, size, flags, self.memory())
    }

    fn unmap_memory(&self) {
        self.device().unmap_memory(self.memory());
    }

    fn free_memory(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        self.device().free_memory(self.memory(), p_allocator);
    }

    fn invalidate_mapped_memory_ranges(&self, mapped_memory_range: &[VkMappedMemoryRange]) {
        self.device().invalidate_mapped_memory_ranges(mapped_memory_range.len() as u32, mapped_memory_range.as_ptr());
    }

    fn bind_buffer_memory(&self, offset: VkDeviceSize) {}
    fn bind_image_memory(&self, offset: VkDeviceSize) {}
}

#[derive(Debug)]
struct Data<T> {
    ptr_: Option<*const T>,
    len_: usize
}

impl<T> Data<T> {
    pub fn as_ptr(&self) -> *const T {
        self.ptr_.unwrap()
    }
    pub fn len(&self) -> usize {
        self.len_
    }
}

#[derive(Debug)]
pub struct Buffer<'a, T> {
    device: &'a VkDevice,

    // gpu side
    buffer: VkBuffer,               // buffer
    memory: VkDeviceMemory,         // gpu address

    // cpu side
    data: Data<T>, 
}

impl<'a, T> MemoryFunctions for Buffer<'a, T> {

    fn device(&self) -> &VkDevice {
        self.device
    }

    fn memory(&self) -> &VkDeviceMemory {
        &self.memory
    }

    fn bind_buffer_memory(&self, offset: VkDeviceSize) {
        self.device().bind_buffer_memory(self.buffer, self.memory, offset);
    }
}

impl<'a, T> Drop for Buffer<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.destroy(None);
        }
    }  
}

impl<'a, T> Buffer<'a, T> {
    pub fn new(
        data_: (Option<*const T>, usize),
        flags: VkBufferCreateFlags,
        usage: VkBufferUsageFlags,
        mem_prop_flags: VkMemoryPropertyFlags,
        device: &'a VkDevice,
    ) -> Self {

        let data = Data { ptr_: data_.0, len_: data_.1 };
        let info = VkBufferCreateInfoBuilder::new()
            .flags(flags)
            .size((data.len() * std::mem::size_of::<T>()) as u64)
            .usage(usage)
            .sharing_mode(VK_SHARING_MODE_EXCLUSIVE)
            .build();

        let buffer = device.create_buffer(&info, None);
        let memory: VkDeviceMemory = memory_function::allocate_buffer_memory(device, buffer, mem_prop_flags);

        Self { device, buffer, memory, data }
    }

    pub fn destroy(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        self.device.destroy_buffer(self.buffer, p_allocator);
        self.device.free_memory(&self.memory, p_allocator);
    }

    pub fn into_raw_vk(&self) -> VkBuffer {
        *&self.buffer
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
            std::ptr::copy_nonoverlapping(
                self.data.as_ptr(), mapped.cast(), self.data.len());
        }
        self.unmap_memory();
    }

    pub fn map_to_cpu_and_unmap(&mut self) -> Vec<T>
    where
        T: std::clone::Clone + Default,
    {
        let mut output = vec![T::default(); self.data.len()];

        let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(mapped.cast(), output.as_mut_ptr(), self.data.len());
        }
        self.unmap_memory();

        output
    }

    pub fn vksize(&self) -> VkDeviceSize {
        (self.data.len() * std::mem::size_of::<T>()) as VkDeviceSize
    }
}

pub trait TextureOperator {

}

pub struct Texture<'a> {
    image: VkImage,             // memory layout
    buffer: VkBuffer,           // staging buffer
    memory: VkDeviceMemory,     // real memory

    device: &'a VkDevice,
}

impl<'a> Texture<'a> {
    pub fn new(
        mem_prop_flags: VkMemoryPropertyFlagBits,
        device: &'a VkDevice,
    // ) -> Self {
    ) {
        // initialize method

        // let mut texture = Self {
        //     image: std::ptr::null_mut(),
        //     memory: std::ptr::null_mut(),

        //     device: device,
        // };

        // texture.allocate_memory(mem_prop_flags);
        // texture
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

    fn memory(&self) -> &VkDeviceMemory {
        &self.memory
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
