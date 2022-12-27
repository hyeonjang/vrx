#include "vkcholesky.h"

#include <stdlib.h>

// global variables
VkPhysicalDevice g_physicalDevice;
VkDevice         g_device;
uint32_t         g_queueFamillyIndex;
VkQueue          g_queue;
VkCommandPool    g_commandPool;

// 
// Context
// 
Context::Context()
:instance(nullptr), physical_devices(nullptr), num_physical_devices(0) {
    VkApplicationInfo app_info = {};
    app_info.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    app_info.pApplicationName = "vk-cholesky";
    app_info.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    app_info.pEngineName = "No Engine";
    app_info.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    app_info.apiVersion = VK_API_VERSION_1_2;

    const char* layers[] ={
        "VK_LAYER_KHRONOS_validation"
    };

    const char* extensions[] ={
        "VK_EXT_debug_report"
    };

    // instance
    {
        VkInstanceCreateInfo instance_create_info{};
        instance_create_info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        instance_create_info.pApplicationInfo = &app_info;
        instance_create_info.enabledLayerCount = 1;
        instance_create_info.ppEnabledLayerNames = layers;
        instance_create_info.enabledExtensionCount = 1;
        instance_create_info.ppEnabledExtensionNames = extensions;
        vkCreateInstance(&instance_create_info, nullptr, &instance);
    }

    // physcial device
    {
        vkEnumeratePhysicalDevices(instance, &num_physical_devices, nullptr);
        physical_devices = (VkPhysicalDevice*)malloc(sizeof(VkPhysicalDevice)*num_physical_devices);
        vkEnumeratePhysicalDevices(instance, &num_physical_devices, physical_devices);
    }
}

std::unique_ptr<Context> Context::ctx = nullptr;

std::unique_ptr<Context> Context::get() {
    if(ctx == nullptr) {
        ctx = std::unique_ptr<Context>(new Context());
    }
    return std::move(ctx);
}

// 
// Device
// 
Device::Device():self(nullptr),queue(nullptr),queue_family_index(0){};
std::unique_ptr<Device> Device::new_compute_device() {

    auto& physical_device = Context::get()->physical_devices[0];
    
    Device* device = new Device();
    // queue family index
    {
        uint32_t queue_family_count = 0;
        vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, nullptr);
        
        VkQueueFamilyProperties* queue_family_properties = new VkQueueFamilyProperties[queue_family_count];
        vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, queue_family_properties);
        for(uint32_t i=0; i<queue_family_count; i++) {
            if(queue_family_properties[i].queueFlags & VK_QUEUE_COMPUTE_BIT) {
                    device->queue_family_index = i;
            }
        }
        delete queue_family_properties;
    }

    // device
    {
        float queue_priority = 1.0;
        VkDeviceQueueCreateInfo device_queue_create_info {};
        device_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        device_queue_create_info.queueFamilyIndex = device->queue_family_index;
        device_queue_create_info.queueCount = 1;
        device_queue_create_info.pQueuePriorities = &queue_priority;
        // device_queue_create_info.flags = VkDeviceQueueCreateFlagBits::VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT;
    
        VkPhysicalDeviceFeatures device_features{};
        VkDeviceCreateInfo device_create_info{};
        device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        device_create_info.queueCreateInfoCount = 1;
        device_create_info.pQueueCreateInfos = &device_queue_create_info;
        device_create_info.pEnabledFeatures = &device_features;
        device_create_info.enabledLayerCount = 0;
        device_create_info.enabledExtensionCount = 0;

        vkCreateDevice(physical_device, &device_create_info, nullptr, &device->self);
        vkGetDeviceQueue(device->self, device->queue_family_index, 0, &device->queue);
    }

    // command pool
    {
        VkCommandPoolCreateInfo cmd_pool_create_info ={};
        cmd_pool_create_info.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
        cmd_pool_create_info.queueFamilyIndex = device->queue_family_index;
        cmd_pool_create_info.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
        vkCreateCommandPool(device->self, &cmd_pool_create_info, nullptr, &device->command_pool);
    }
    return std::unique_ptr<Device>(device);
}

std::unique_ptr<Buffer> Device::create_buffer(const VkBufferCreateInfo info, size_t size) const {
    auto buffer = std::make_unique<Buffer>(info, size, &self);
    return buffer;
}

std::vector<CommandBuffer> Device::allocate_command_buffer(const VkCommandBufferAllocateInfo info) const {
    
    // auto cmdBuffer = std::vector<CommandBuffer>(info.commandBufferCount);
    VkCommandBuffer* cmdBuffers = new VkCommandBuffer[info.commandBufferCount];
    vkAllocateCommandBuffers(self, &info, cmdBuffers);

    return std::vector<CommandBuffer>((CommandBuffer*)cmdBuffers, (CommandBuffer*)cmdBuffers+info.commandBufferCount);
}

// 
// VxBuffer
// 
Buffer::Buffer(const VkBufferCreateInfo info, size_t _size, const VkDevice* _p_device)
:size(_size), p_device(_p_device) {
    vkCreateBuffer(*p_device, &info, nullptr, &self);
}

void Buffer::alloc(const VkMemoryPropertyFlags memPropFlags) {

    //@@ here maybe moved to Context
    auto& phyiscal_device = Context::get()->physical_devices[0];
    VkPhysicalDeviceMemoryProperties mem_prop;
    vkGetPhysicalDeviceMemoryProperties(phyiscal_device, &mem_prop);
    VkMemoryRequirements mem_req;
    vkGetBufferMemoryRequirements(*p_device, self, &mem_req);
    
    VkMemoryAllocateInfo info {};
    info.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    info.allocationSize = mem_req.size;

    for(uint32_t i=0; i<mem_prop.memoryTypeCount; i++) {
        if((mem_req.memoryTypeBits&1)==1) {
            if((mem_prop.memoryTypes[i].propertyFlags&memPropFlags) == memPropFlags) {
                info.memoryTypeIndex = i;
            }
        }
    }
    vkAllocateMemory(*p_device, &info, nullptr, &memory);
}

void Buffer::map(void* _data) {
    vkMapMemory(g_device, memory, 0, size, 0, &_data);
    memcpy(_data, data, size);
    vkUnmapMemory(g_device, memory);
}

void Buffer::bind() {
    vkBindBufferMemory(g_device, self, memory, 0);
}

// 
// VxCommandBuffer
// 
void CommandBuffer::begin() {
    VkCommandBufferBeginInfo info = {};
    info.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;

    vkBeginCommandBuffer(self, &info);
}

void CommandBuffer::end() {
    vkEndCommandBuffer(self);
}

void CommandBuffer::copyBuffer(VkBuffer src, VkBuffer dst, uint32_t regionCount, const VkBufferCopy* copy) {
    vkCmdCopyBuffer(self, src, dst, regionCount, copy);
}

void CommandBuffer::bindPipeline(VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline) {
    vkCmdBindPipeline(self, pipelineBindPoint, pipeline);
}

//
//
//
Descriptor::Descriptor(uint32_t _count)
:count(_count),pool(VkDescriptorPool())
,sets(new VkDescriptorSet[count])
,setLayouts(new VkDescriptorSetLayout[count]) {

    this->create_descriptor_pool();
    this->create_descriptor_layout();
    this->allocate_descriptor_set();
}
Descriptor::~Descriptor(){
    delete[] sets;
    delete[] setLayouts;
}

void Descriptor::create_descriptor_pool() {
    VkDescriptorPoolSize poolSize = {};
    poolSize.type = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    poolSize.descriptorCount = 1;

    VkDescriptorPoolCreateInfo createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    createInfo.poolSizeCount = 1;
    createInfo.pPoolSizes = &poolSize;
    createInfo.maxSets = 1;
    vkCreateDescriptorPool(g_device, &createInfo, nullptr, &pool);
}

void Descriptor::create_descriptor_layout() {
    VkDescriptorSetLayoutBinding binding = {};
    binding.descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    binding.stageFlags = VK_SHADER_STAGE_COMPUTE_BIT;
    binding.binding = 0;
    binding.descriptorCount = count;

    VkDescriptorSetLayoutCreateInfo createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    createInfo.bindingCount = 1;
    createInfo.pBindings = &binding;
    vkCreateDescriptorSetLayout(g_device, &createInfo, nullptr, setLayouts);
}

void Descriptor::allocate_descriptor_set() {
    VkDescriptorSetAllocateInfo desc_set_alloc_info {};
    desc_set_alloc_info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    desc_set_alloc_info.descriptorPool = pool;
    desc_set_alloc_info.pSetLayouts = setLayouts;
    desc_set_alloc_info.descriptorSetCount = count;
    vkAllocateDescriptorSets(g_device, &desc_set_alloc_info, sets); 
}

void Descriptor::updateDescriptorSets(const VkDescriptorBufferInfo info, size_t index) {

    if(index > this->count - 1) {
        // runtime error
    }

    VkWriteDescriptorSet write_desc_set = {};
    write_desc_set.sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    write_desc_set.descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    write_desc_set.dstSet = sets[index];
    write_desc_set.pBufferInfo = &info;
    write_desc_set.pImageInfo = nullptr;
    write_desc_set.descriptorCount = count;
    write_desc_set.dstBinding = 0;
    vkUpdateDescriptorSets(g_device, 1, &write_desc_set, 0, NULL);
}

//
// VxComputePipeline
//
ComputePipeline::ComputePipeline() 
:pipelineLayout(VkPipelineLayout())
,pipeline(VkPipeline()){}

void ComputePipeline::createPipelineLayout(Descriptor descriptor) {

    VkPipelineLayoutCreateInfo info {};
    info.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    info.pSetLayouts = descriptor.setLayouts;
    info.setLayoutCount = descriptor.count;
    // currently 
    info.pPushConstantRanges = nullptr;
    info.pushConstantRangeCount = 0;

    vkCreatePipelineLayout(g_device, &info, nullptr, &pipelineLayout);
}

void ComputePipeline::createPipeline() {

    VkComputePipelineCreateInfo info {};
    info.sType = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO;
    // info.stage
    info.layout = pipelineLayout;
    info.basePipelineHandle = VK_NULL_HANDLE;
    info.basePipelineIndex = 0;

    vkCreateComputePipelines(g_device, VK_NULL_HANDLE, 1, &info, nullptr, &pipeline);
}