#[macro_use]
use crate::*;

pub mod buffer;
pub mod descriptor;
pub mod texture;

pub use buffer::*;
pub use descriptor::*;
pub use texture::*;

use func_static::vk_instantiate;

mod memory_function {

    use crate::*;

    /// functional
    fn get_phyiscal_device_memory_property_collect(
        mem_prop_flags: VkMemoryPropertyFlagBits,
    ) -> Vec<u32> {
        let ctx = vulkan_context();

        //@@ to static
        let mut mem_prop = ctx.get_physical_device_memory_properties();
        let mut collect: Vec<u32> = (0..mem_prop.memoryTypeCount).collect();
        collect.retain(|i| {
            mem_prop.memoryTypes[*i as usize].propertyFlags
                == (mem_prop_flags as VkMemoryPropertyFlags)
        });
        collect
    }

    pub fn allocate_buffer_memory(
        device: &VkDevice,
        buffer: VkBuffer,
        mem_prop_flags: VkMemoryPropertyFlagBits,
    ) -> VkDeviceMemory {
        let collect = get_phyiscal_device_memory_property_collect(mem_prop_flags);

        let mut mem_req = device.get_buffer_memory_requirements(buffer);
        let mut mem_alloc_info = VkMemoryAllocateInfoBuilder::new()
            .allocation_size(mem_req.size)
            .memory_type_index(collect[0])
            .build();

        device.allocate_memory(&mem_alloc_info, None)
    }

    pub fn allocate_image_memory(
        device: &VkDevice,
        image: VkImage,
        mem_prop_flags: VkMemoryPropertyFlagBits,
    ) -> VkDeviceMemory {
        let collect = get_phyiscal_device_memory_property_collect(mem_prop_flags);

        let mut mem_req = device.get_image_memory_requirements(image);
        let mut mem_alloc_info = VkMemoryAllocateInfoBuilder::new()
            .allocation_size(mem_req.size)
            .memory_type_index(collect[0])
            .build();

        device.allocate_memory(&mem_alloc_info, None)
    }

    #[inline]
    pub fn to_gpu<T>(device: &VkDevice, gpu: &VkDeviceMemory, cpu: (*const T, usize)) {
        let mapped = device.map_memory(0, cpu.1 as u64, 0, gpu).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(cpu.0, mapped.cast(), cpu.1);
        }
        device.unmap_memory(gpu);
    }
}

#[derive(Debug)]
struct BufferAndMemory(VkBuffer, VkDeviceMemory);
#[derive(Debug)]
struct ImageAndMemory(VkImage, VkDeviceMemory);

pub trait MemoryFunctions {
    fn device(&self) -> &VkDevice;
    fn memory(&self) -> &VkDeviceMemory;

    fn map_memory(
        &self,
        offset: u64,
        size: u64,
        flags: u32,
    ) -> anyhow::Result<*mut std::os::raw::c_void> {
        self.device().map_memory(offset, size, flags, self.memory())
    }

    fn unmap_memory(&self) {
        self.device().unmap_memory(self.memory());
    }

    fn free_memory(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        self.device().free_memory(self.memory(), p_allocator);
    }

    fn invalidate_mapped_memory_ranges(&self, mapped_memory_range: &[VkMappedMemoryRange]) {
        self.device().invalidate_mapped_memory_ranges(
            mapped_memory_range.len() as u32,
            mapped_memory_range.as_ptr(),
        );
    }

    fn bind_memory(&self, offset: VkDeviceSize); // buffer or image
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

type Texture1D<'a, T> = TextureImpl<'a, T, 1>;
type Texture2D<'a, T> = TextureImpl<'a, T, 2>;
type Texture3D<'a, T> = TextureImpl<'a, T, 3>;

pub enum Texture<'a, T> {
    D1(Texture1D<'a, T>),
    D2(Texture2D<'a, T>),
    D3(Texture3D<'a, T>),
}

// pub enum Descriptor<'a> {

// }

pub trait DescriptorFunctions {
    // fn get_descriptor_set_layout_binding(&self) -> VkDescriptorSetLayoutBinding;
    // fn get_write_descriptor_set(&self) -> VkWriteDescriptorSet;
}
