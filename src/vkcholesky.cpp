#include "vkcholesky.hpp"

#include <stdlib.h>

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

    // instance
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
    printf("vulkan init");

    // physcial device
    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(g_instance, &deviceCount, nullptr);

    VkPhysicalDevice* physical_devices = (VkPhysicalDevice*)malloc(sizeof(VkPhysicalDevice)*deviceCount);
    vkEnumeratePhysicalDevices(g_instance, &deviceCount, physical_devices);
    g_physicalDevice = physical_devices[0];

    

    // std::vector<VkPhysicalDevice> physical_devices(deviceCount);
    // VK_ASSERT(vkEnumeratePhysicalDevices(instance, &deviceCount, &physical_devices[0]));
    // physical_device = physical_devices[0];

    // queue family index
    // uint32_t queue_family_count = 0;
    // vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, nullptr);

    // std::vector<VkQueueFamilyProperties> queue_family_properties(queue_family_count);
    // vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, &queue_family_properties[0]);

    // for(uint32_t i=0; i<queue_family_count; i++) {
        // if(queue_family_properties[i].queueFlags & VK_QUEUE_COMPUTE_BIT) {
        // if(queue_family_properties[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) {
            // queue_family_index = i;
            // printf( "queue family index: %d", queue_family_index );
        // }
    // }
    // //queue_family_index = 0;
    // printf( "queue family index: %d\n", queue_family_index );
}

void Buffer::alloc(VkMemoryAllocateInfo info) {

    VkPhysicalDeviceMemoryProperties mem_prop;
    vkGetPhysicalDeviceMemoryProperties(g_physicalDevice, &mem_prop);

    VkMemoryRequirements mem_req;
    vkGetBufferMemoryRequirements(*p_device, self, &mem_req);

    vkAllocateMemory(*p_device, &info, nullptr, &memory);
    vkBindBufferMemory(*p_device, self, memory, 0);
}

void Buffer::map(void* _data) {
    vkMapMemory(*p_device, memory, 0, size, 0, &_data);
    memcpy(_data, data, size);
    vkUnmapMemory(*p_device, memory);
}