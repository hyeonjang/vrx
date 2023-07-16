use crate::memory::*;
use crate::*;

pub struct TextureBuilder<'a, T, const dim: usize> {
    create_info: VkImageCreateInfo,
    mem_prop_flags: VkMemoryPropertyFlagBits,
    data: (Option<*const T>, [u32; dim]),
    device: &'a VkDevice,
}

impl<'a, T, const dim: usize> TextureBuilder<'a, T, dim> {
    pub fn new(data: (Option<*const T>, [u32; dim]), device: &'a VkDevice) -> Self {
        // default create info
        let create_info = VkImageCreateInfoBuilder::new()
            .mip_levels(1)
            .array_layers(1)
            .samples(VK_SAMPLE_COUNT_1_BIT)
            .build();

        let mem_prop_flags = VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT;

        Self {
            create_info,
            mem_prop_flags,
            data,
            device,
        }
    }

    pub fn flags(mut self, flags: VkImageCreateFlagBits) -> Self {
        self.create_info.flags = flags as VkImageCreateFlags;
        self
    }

    pub fn format(mut self, format: VkFormat) -> Self {
        self.create_info.format = format;
        self
    }

    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.create_info.mipLevels = mip_levels;
        self
    }

    pub fn array_layers(mut self, array_layers: u32) -> Self {
        self.create_info.arrayLayers = array_layers;
        self
    }

    pub fn samples(mut self, samples: VkSampleCountFlagBits) -> Self {
        self.create_info.samples = samples;
        self
    }

    pub fn tiling(mut self, tiling: VkImageTiling) -> Self {
        self.create_info.tiling = tiling;
        self
    }

    pub fn usage(mut self, usage: VkImageUsageFlagBits) -> Self {
        self.create_info.usage = usage as VkImageUsageFlags;
        self
    }

    pub fn build(mut self) -> TextureImpl<'a, T, dim> {
        let mut shape = [1, 1, 1];
        for i in 0..dim {
            shape[i] = self.data.1[i]
        }

        self.create_info.extent = VkExtent3D {
            width: shape[0],
            height: shape[1],
            depth: shape[2],
        };

        match dim {
            1 => {
                self.create_info.imageType = VkImageType::VK_IMAGE_TYPE_1D;
                Texture::D1(TextureImpl::new(
                    (self.data.0, [self.data.1[0]]),
                    self.create_info,
                    self.mem_prop_flags,
                    self.device,
                ))
            }
            2 => {
                self.create_info.imageType = VkImageType::VK_IMAGE_TYPE_2D;
                Texture::D2(TextureImpl::new(
                    (self.data.0, [self.data.1[0], self.data.1[1]]),
                    self.create_info,
                    self.mem_prop_flags,
                    self.device,
                ))
            }
            3 => {
                self.create_info.imageType = VkImageType::VK_IMAGE_TYPE_3D;
                Texture::D3(TextureImpl::new(
                    (
                        self.data.0,
                        [self.data.1[0], self.data.1[1], self.data.1[2]],
                    ),
                    self.create_info,
                    self.mem_prop_flags,
                    self.device,
                ))
            }
            _ => todo!(),
        };

        TextureImpl::new(
            self.data,
            self.create_info,
            self.mem_prop_flags,
            self.device,
        )
    }
}

pub fn texture_builder_from_image(
    image: std::fs::File,
    device: &VkDevice,
) -> TextureBuilder<'_, u8, 2> {
    let decoder = png::Decoder::new(image);
    let mut reader = decoder.read_info().unwrap();

    let info = reader.info();
    let (width, height) = info.size();
    let mut pixels = vec![0; reader.info().raw_bytes()];
    reader.next_frame(&mut pixels).unwrap();

    let (color_type, bit_depth) = reader.output_color_type();

    // png assumption
    let format = match (color_type, bit_depth) {
        (png::ColorType::Rgb, png::BitDepth::Eight) => VkFormat::VK_FORMAT_R32G32B32A32_SINT,
        (png::ColorType::Rgba, png::BitDepth::Eight) => VkFormat::VK_FORMAT_R32G32B32A32_SINT,
        _ => VkFormat::VK_FORMAT_R32G32B32A32_SINT,
    };

    let mut builder = TextureBuilder::new((Some(pixels.as_ptr()), [width, height]), device);
    // builder = builder.format(format);
    builder
}

pub fn texture_builder_from_path<'a>(
    path: &'static str,
    device: &'a VkDevice,
) -> TextureBuilder<'a, u8, 2> {
    let image = std::fs::File::open(path).unwrap();
    texture_builder_from_image(image, device)
}

struct TData<T, const dim: usize> {
    ptr_: Option<*const T>,
    len_: [u32; dim],
}

impl<T, const dim: usize> TData<T, dim> {
    pub fn as_ptr(&self) -> *const T {
        self.ptr_.unwrap()
    }

    pub fn shape(&self) -> [u32; dim] {
        self.len_
    }

    pub fn len(&self) -> usize {
        self.len_.iter().product::<u32>() as usize
    }
}

pub struct TextureImpl<'a, T, const dim: usize> {
    device: &'a VkDevice,
    gpu: ImageAndMemory,
    gpu_stage: BufferAndMemory,
    cpu: TData<T, dim>,
    info: VkImageCreateInfo,
}

impl<'a, T, const dim: usize> TextureImpl<'a, T, dim> {
    fn new(
        data_: (Option<*const T>, [u32; dim]),
        info: VkImageCreateInfo,
        mem_prop_flags: VkMemoryPropertyFlagBits,
        device: &'a VkDevice,
    ) -> Self {
        let cpu = TData {
            ptr_: data_.0,
            len_: data_.1,
        };

        let buffer_create_info = VkBufferCreateInfoBuilder::new()
            .usage(VK_BUFFER_USAGE_TRANSFER_SRC_BIT as VkBufferUsageFlags)
            .size((cpu.len() * 4 * std::mem::size_of::<T>()) as u64)
            .build();

        let buffer = device.create_buffer(&buffer_create_info, None);
        let buf_memory = memory_function::allocate_buffer_memory(
            device,
            buffer,
            VK_MEMORY_PROPERTY_HOST_COHERENT_BIT | VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT,
        );
        memory_function::to_gpu(device, &buf_memory, (cpu.as_ptr(), cpu.len()));

        device.bind_buffer_memory(buffer, buf_memory, 0);
        let gpu_stage = BufferAndMemory(buffer, buf_memory);

        let image = device.create_image(&info, None);
        let img_memory = memory_function::allocate_image_memory(device, image, mem_prop_flags);

        device.bind_image_memory(image, img_memory, 0);
        let gpu = ImageAndMemory(image, img_memory);

        Self {
            device,
            gpu,
            cpu,
            gpu_stage,
            info,
        }
    }

    pub fn transition_image_layout() {}

    pub fn cmd_copy_buffer_to_image(&self, command_pool: VkCommandPool) {
        let info = VkCommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .level(VkCommandBufferLevel(0))
            .command_buffer_count(1)
            .build();

        let cmds = (*self.device).allocate_command_buffers(&info);

        vkCmdBlock! {
            THIS cmds[0];

            let subresource = VkImageSubresourceLayers {
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                mipLevel: 0,
                baseArrayLayer: 0,
                layerCount: 1
            };

            let region = VkBufferImageCopyBuilder::new()
                .image_subresource(subresource)
                .image_offset(VkOffset3D { x:0, y:0, z:0 })
                .image_extent(self.info.extent)
                .build();

            COPY_BUFFER_TO_IMAGE(self.gpu_stage.0, self.gpu.0, VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region);
        };
    }

    // If want to generate same view with the current
    pub fn make_view(&self) -> VkImageView {
        //@@TODO should be consideration
        let subresource_range = VkImageSubresourceRangeBuilder::new()
            .aspect_mask(VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let mut builder = VkImageViewCreateInfoBuilder::new()
            .image(self.gpu.0)
            .view_type(VkImageViewType::VK_IMAGE_VIEW_TYPE_2D)
            .format(self.info.format)
            .subresource_range(subresource_range);

        match dim {
            1 => {
                builder = builder.view_type(VkImageViewType::VK_IMAGE_VIEW_TYPE_1D);
            }
            2 => {
                builder = builder.view_type(VkImageViewType::VK_IMAGE_VIEW_TYPE_2D);
            }
            3 => {
                builder = builder.view_type(VkImageViewType::VK_IMAGE_VIEW_TYPE_3D);
            }
            _ => todo!(),
        };
        let info = builder.build();

        self.device.create_image_view(&info, None)
    }
}
