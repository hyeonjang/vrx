use crate::memory::*;
use crate::*;

#[derive(Debug)]
struct Data<T> {
    ptr_: Option<*const T>,
    len_: usize,
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
    gpu: BufferAndMemory,
    cpu: Data<T>,
}

impl<'a, T> MemoryFunctions for Buffer<'a, T> {
    fn device(&self) -> &VkDevice {
        self.device
    }

    fn memory(&self) -> &VkDeviceMemory {
        &self.gpu.1
    }

    fn bind_memory(&self, offset: VkDeviceSize) {
        self.device()
            .bind_buffer_memory(self.gpu.0, self.gpu.1, offset);
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
        flags: VkBufferCreateFlagBits,
        usage: VkBufferUsageFlagBits,
        mem_prop_flags: VkMemoryPropertyFlagBits,
        device: &'a VkDevice,
    ) -> Self {
        let cpu = Data {
            ptr_: data_.0,
            len_: data_.1,
        };

        let info = VkBufferCreateInfoBuilder::new()
            .flags(flags as VkBufferCreateFlags)
            .size((cpu.len() * std::mem::size_of::<T>()) as u64)
            .usage(usage as VkBufferUsageFlags)
            .sharing_mode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
            .build();

        let buffer = device.create_buffer(&info, None);
        let memory: VkDeviceMemory =
            memory_function::allocate_buffer_memory(device, buffer, mem_prop_flags);
        device.bind_buffer_memory(buffer, memory, 0);

        let gpu = BufferAndMemory(buffer, memory);

        Self { device, gpu, cpu }
    }

    pub fn destroy(&self, p_allocator: Option<*const VkAllocationCallbacks>) {
        self.device.destroy_buffer(self.gpu.0, p_allocator);
        self.device.free_memory(&self.gpu.1, p_allocator);
    }

    pub fn into_raw_vk(&self) -> VkBuffer {
        self.gpu.0
    }

    // mappings
    pub fn map(&mut self, len: usize, data: *const T) {
        let mapped = self
            .map_memory(0, (std::mem::size_of::<T>() * len) as u64, 0)
            .unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(data, mapped.cast(), len);
        }
        self.unmap_memory();
    }

    pub fn map_to_gpu_and_unmap(&self) {
        let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(self.cpu.as_ptr(), mapped.cast(), self.cpu.len());
        }
        self.unmap_memory();
    }

    pub fn map_to_cpu_and_unmap(&mut self) -> Vec<T>
    where
        T: std::clone::Clone + Default,
    {
        let mut output = vec![T::default(); self.cpu.len()];

        let mapped = self.map_memory(0, self.vksize(), 0).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(mapped.cast(), output.as_mut_ptr(), self.cpu.len());
        }
        self.unmap_memory();

        output
    }

    pub fn vksize(&self) -> VkDeviceSize {
        (self.cpu.len() * std::mem::size_of::<T>()) as VkDeviceSize
    }
}

pub trait DescriptorStruct {
    fn get_type(&self) -> VkDescriptorType;
    fn get_set(&self) -> u32;
    fn get_binding(&self) -> u32;
}

pub trait DescriptorTrait {
    fn layout_binding(&self) -> VkDescriptorSetLayoutBinding;
}

impl<'a, T: DescriptorStruct> DescriptorTrait for Buffer<'a, T> {
    fn layout_binding(&self) -> VkDescriptorSetLayoutBinding {
        
        let generic_instance = unsafe { &*self.cpu.as_ptr() };
        
        VkDescriptorSetLayoutBindingBuilder::new()
            .binding(unsafe { T::get_binding(generic_instance) })
            .descriptor_type(unsafe { T::get_type(generic_instance) })
            // .descriptor_count()
            // .stage_flags()
            .build()
    }

    // fn set_layout_binding(&self) -> VkDescriptorSetLayoutBinding {

    // }

    // fn get_descriptor_set_layout_binding(&self) -> VkDescriptorSetLayoutBinding {

    //     VkDescriptorSetLayoutBindingBuilder::new()
    //         .binding(self.id)
    //         .descriptor_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)
    //         .descriptor_count(1)
    //         .stage_flags(VK_SHADER_STAGE_VERTEX_BIT as VkShaderStageFlags)
    //         .build()
    // }

    // fn get_write_descriptor_set(&self) -> VkWriteDescriptorSet {
    //     VkWriteDescriptorSetBuilder::new().build()
    // }
}

pub type Descriptor<'a> = Box<dyn DescriptorTrait + 'a>;
