#include "vkcholesky.hpp"

#include <stdlib.h>

VkDevice         g_device;

void initVulkan() {
    VkApplicationInfo app_info ={};
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
    printf("vulkan init");
    // instance
    {
        VkInstanceCreateInfo instance_create_info{};
        instance_create_info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        instance_create_info.pApplicationInfo = &app_info;
        instance_create_info.enabledLayerCount = 1;
        instance_create_info.ppEnabledLayerNames = layers;
        instance_create_info.enabledExtensionCount = 1;
        instance_create_info.ppEnabledExtensionNames = extensions;
        // uint32_t glfwExtensionCount = 0;
        // const char** glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);
        // instance_create_info.enabledLayerCount = 0;
        // instance_create_info.ppEnabledLayerNames = VK_NULL_HANDLE;
        // instance_create_info.enabledExtensionCount = glfwExtensionCount;
        // instance_create_info.ppEnabledExtensionNames = glfwExtensions;
        vkCreateInstance(&instance_create_info, nullptr, &g_instance);
    }

    // physcial device
    {
        uint32_t deviceCount = 0;
        vkEnumeratePhysicalDevices(g_instance, &deviceCount, nullptr);

        VkPhysicalDevice* physical_devices = (VkPhysicalDevice*)malloc(sizeof(VkPhysicalDevice)*deviceCount);
        vkEnumeratePhysicalDevices(g_instance, &deviceCount, physical_devices);
        g_physicalDevice = physical_devices[0];
        delete physical_devices;
    }

    // queue family index
    {
        uint32_t queue_family_count = 0;
        vkGetPhysicalDeviceQueueFamilyProperties(g_physicalDevice, &queue_family_count, nullptr);
        
        VkQueueFamilyProperties* queue_family_properties = new VkQueueFamilyProperties[queue_family_count];
        vkGetPhysicalDeviceQueueFamilyProperties(g_physicalDevice, &queue_family_count, queue_family_properties);
        for(uint32_t i=0; i<queue_family_count; i++) {
            if(queue_family_properties[i].queueFlags & VK_QUEUE_COMPUTE_BIT) {
                // if(queue_family_properties[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) {
                    g_queueFamillyIndex = i;
                    printf( "queue family index: %d", g_queueFamillyIndex );
                // }
            }
        }
        delete queue_family_properties;
    }

    // device
    {
        float queue_priority = 1.0;
        VkDeviceQueueCreateInfo device_queue_create_info {};
        device_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        device_queue_create_info.queueFamilyIndex = g_queueFamillyIndex;
        device_queue_create_info.queueCount = 1;
        device_queue_create_info.pQueuePriorities = &queue_priority;
        // device_queue_create_info.flags = VkDeviceQueueCreateFlagBits::VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT;
    
        VkPhysicalDeviceFeatures device_features{};
        VkDeviceCreateInfo device_create_info {};
        device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        device_create_info.queueCreateInfoCount = 1;
        device_create_info.pQueueCreateInfos = &device_queue_create_info;
        device_create_info.pEnabledFeatures = &device_features;
        device_create_info.enabledLayerCount = 0;
        device_create_info.enabledExtensionCount = 0;
        vkCreateDevice(g_physicalDevice, &device_create_info, nullptr, &g_device);
        vkGetDeviceQueue(g_device, g_queueFamillyIndex, 0, &g_queue);
    }

    // vk command pool
    {
        VkCommandPoolCreateInfo cmd_pool_create_info ={};
        cmd_pool_create_info.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
        cmd_pool_create_info.queueFamilyIndex = g_queueFamillyIndex;
        cmd_pool_create_info.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
        vkCreateCommandPool(g_device, &cmd_pool_create_info, nullptr, &g_commandPool);
    }
}

// 
// VxBuffer
// 
void Buffer::alloc(VkMemoryAllocateInfo info) {

    VkPhysicalDeviceMemoryProperties mem_prop;
    vkGetPhysicalDeviceMemoryProperties(g_physicalDevice, &mem_prop);

    VkMemoryRequirements mem_req;
    vkGetBufferMemoryRequirements(g_device, self, &mem_req);

    vkAllocateMemory(g_device, &info, nullptr, &memory);
    vkBindBufferMemory(g_device, self, memory, 0);
}

void Buffer::map(void* _data) {
    vkMapMemory(g_device, memory, 0, size, 0, &_data);
    memcpy(_data, data, size);
    vkUnmapMemory(g_device, memory);
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
Descriptor::Descriptor() {
    this->create_descriptor_pool();
    this->create_descriptor_layout();
    this->allocate_descriptor_set();
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


//
// VxComputePipeline;
//

// void ComputePipeline:: 