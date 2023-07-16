#[macro_use]
use crate::*;

#[inline]
pub fn set_layout_binding(
    binding: u32,
    descriptor_type: VkDescriptorType,
    descriptor_count: u32,
    stage_flag: VkShaderStageFlagBits,
) -> VkDescriptorSetLayoutBinding {
    VkDescriptorSetLayoutBinding {
        binding: binding,
        descriptorType: descriptor_type,
        descriptorCount: 1,
        stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as u32,
        pImmutableSamplers: null(),
    }
}

#[inline]
pub fn set_layout_bindings<const N: usize>(
    inputs: [(u32, VkDescriptorType, u32, VkShaderStageFlagBits); N],
) -> [VkDescriptorSetLayoutBinding; N] {
    let mut output = [VkDescriptorSetLayoutBindingBuilder::new().build(); N];

    for i in 0..N {
        output[i] = VkDescriptorSetLayoutBinding {
            binding: inputs[i].0,
            descriptorType: inputs[i].1,
            descriptorCount: inputs[i].2,
            stageFlags: inputs[i].3 as u32,
            pImmutableSamplers: null(),
        }
    }

    output
}

pub struct ResourceBinding<'a> {
    pub descriptor_pool: VkDescriptorPool,
    pub descriptor_set_layouts: VkDescriptorSetLayout,
    pub descriptor_sets: Vec<VkDescriptorSet>,
    device: &'a VkDevice,
}

impl<'a> ResourceBinding<'a> {
    pub fn new(bindings: &[VkDescriptorSetLayoutBinding], device: &'a VkDevice) -> Self {
        let pool_sizes = Self::descriptor_pool_size(bindings);
        let descriptor_pool = Self::create_descriptor_pool(pool_sizes, device);

        let desc_set_layouts_create_info = VkDescriptorSetLayoutCreateInfoBuilder::new()
            .binding_count(bindings.len() as u32)
            .p_bindings(bindings.as_ptr())
            .build();

        let descriptor_set_layouts =
            device.create_descriptor_set_layout(&desc_set_layouts_create_info, None);

        let desc_set_allocate_info = VkDescriptorSetAllocateInfoBuilder::new()
            .descriptor_pool(descriptor_pool)
            .descriptor_set_count(bindings.len() as u32)
            .p_set_layouts(&descriptor_set_layouts)
            .build();

        let descriptor_sets = device.allocate_descriptor_sets(&desc_set_allocate_info);

        Self {
            descriptor_pool,
            descriptor_set_layouts,
            descriptor_sets,
            device,
        }
    }

    // descriptor pool
    fn descriptor_pool_size(
        bindings: &[VkDescriptorSetLayoutBinding],
    ) -> Vec<VkDescriptorPoolSize> {
        let mut hmap: HashMap<VkDescriptorType, u32> = HashMap::new();

        bindings.iter().for_each(|binding| {
            *hmap.entry(binding.descriptorType).or_insert(0) += 1;
        });

        let desc_pool_size = hmap
            .iter()
            .map(|(key, value)| VkDescriptorPoolSize {
                type_: *key,
                descriptorCount: *value,
            })
            .collect();

        desc_pool_size
    }

    fn create_descriptor_pool(
        pool_sizes: Vec<VkDescriptorPoolSize>,
        device: &'a VkDevice,
    ) -> VkDescriptorPool {
        let max_sets = pool_sizes
            .iter()
            .fold(0, |init, pool_size| init + pool_size.descriptorCount);

        let info = VkDescriptorPoolCreateInfoBuilder::new()
            .pool_size_count(pool_sizes.len() as u32)
            .p_pool_sizes(pool_sizes.as_ptr())
            .max_sets(max_sets)
            .build();

        device.create_descriptor_pool(&info, None)
    }

    // update
    pub fn update(&self, desc_writes: &[VkWriteDescriptorSet]) {
        self.device.update_descriptor_sets(desc_writes, &[]);
    }

    pub fn copy(&self, desc_copies: &[VkCopyDescriptorSet]) {
        self.device.update_descriptor_sets(&[], desc_copies);
    }

    pub fn update_and_copy(
        &self,
        desc_writes: &[VkWriteDescriptorSet],
        desc_copies: &[VkCopyDescriptorSet],
    ) {
        self.device.update_descriptor_sets(desc_writes, desc_copies);
    }
}
