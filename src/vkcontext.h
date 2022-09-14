#pragma once
#include <stdio.h>
#include <stdlib.h>
#include <memory>
#include "vulkan/vulkan.h"

#include <vector>

// #define GLFW_INCLUDE_VULKAN
// #include <GLFW/glfw3.h>

struct vkcontext_t {
    vkcontext_t();
    std::unique_ptr<VkDevice> get_device();

    VkInstance instance; // VK_NULL_HANDLE
    VkPhysicalDevice physical_device;
    uint32_t queue_family_index;
};

vkcontext_t::vkcontext_t() {
    VkApplicationInfo app_info;
    app_info.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    app_info.pApplicationName = "vk-cholesky";
    app_info.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    app_info.pEngineName = "No Engine";
    app_info.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    app_info.apiVersion = VK_API_VERSION_1_0;

    // instance
    VkInstanceCreateInfo instance_create_info{};
    instance_create_info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    instance_create_info.pApplicationInfo = &app_info;
    
    // uint32_t glfwExtensionCount = 0;
    // const char** glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);
    // instance_create_info.enabledLayerCount = 0;
    // instance_create_info.ppEnabledLayerNames = VK_NULL_HANDLE;
    // instance_create_info.enabledExtensionCount = glfwExtensionCount;
    // instance_create_info.ppEnabledExtensionNames = glfwExtensions;
    VkResult result = vkCreateInstance(&instance_create_info, nullptr, &instance);

    // physcial device
    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);

    VkPhysicalDevice* physical_devices = (VkPhysicalDevice*)malloc(sizeof(VkPhysicalDevice)*deviceCount);
    vkEnumeratePhysicalDevices(instance, &deviceCount, physical_devices);

    physical_device = physical_devices[0];

    // queue family index
    uint32_t queue_family_count = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, nullptr);

    std::vector<VkQueueFamilyProperties> queue_family_properties(queue_family_count);
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, &queue_family_properties[0]);

    for(uint32_t i=0; i<queue_family_count; i++) {
        if(queue_family_properties[i].queueFlags & VK_QUEUE_COMPUTE_BIT) {
            queue_family_index = i;
        }
    }
}

std::unique_ptr<VkDevice> vkcontext_t::get_device() {

    float queue_priority = 1.0;

    VkDeviceQueueCreateInfo device_queue_create_info;
    device_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    device_queue_create_info.queueFamilyIndex = queue_family_index;
    device_queue_create_info.queueCount = 1;
    device_queue_create_info.pQueuePriorities = &queue_priority;

    VkPhysicalDeviceFeatures device_features;

    VkDeviceCreateInfo device_create_info;
    device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    device_create_info.queueCreateInfoCount = 1;
    device_create_info.pQueueCreateInfos = &device_queue_create_info;
    device_create_info.pEnabledFeatures = &device_features;

    VkDevice device;
    VkResult result = vkCreateDevice(physical_device, &device_create_info, nullptr, &device);

    // return std::unique_ptr<VkDevice>(p_device);
}

struct vk_compute_pipeline_t {

    vk_compute_pipeline_t(VkDevice* device);

    void create_shader_module();

    VkPipeline pipeline;
    const VkDevice* p_device;
};

static size_t read_file_length(const char* filepath) {

    return 0;
}

static uint32_t* read_file(const char* filepath) {

    uint32_t* buffer;
    FILE* fp = fopen(filepath, "r");



    fclose(fp);

    return buffer;
}

void vk_compute_pipeline_t::create_shader_module() {

    VkShaderModuleCreateInfo info;
    info.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    info.codeSize = read_file_length("./src/shader/cholesky.spv");
    info.pCode = read_file("./src/shader/cholesky");

    VkShaderModule shadermodule;
    VkResult result = vkCreateShaderModule(*p_device, &info, nullptr, &shadermodule);
    // return shadermodule;
}

vk_compute_pipeline_t::vk_compute_pipeline_t(VkDevice* device) {
        
    VkPipelineShaderStageCreateInfo comp_shader_info;
    comp_shader_info.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    comp_shader_info.stage = VK_SHADER_STAGE_COMPUTE_BIT;
    // comp_shader_info.module = create_shader_module()
    comp_shader_info.pName = "main";

    VkPipelineLayoutCreateInfo layout_info;
    layout_info.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    layout_info.setLayoutCount = 0;
    layout_info.pSetLayouts = nullptr;
    layout_info.pushConstantRangeCount = 0;
    layout_info.pPushConstantRanges = nullptr;
    
    VkPipelineLayout layout;
    vkCreatePipelineLayout(*p_device, &layout_info, nullptr, &layout);

    VkComputePipelineCreateInfo comp_pipeline_create_info;
    comp_pipeline_create_info.sType = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO;
    comp_pipeline_create_info.stage = comp_shader_info;
    comp_pipeline_create_info.basePipelineHandle = VK_NULL_HANDLE;
    comp_pipeline_create_info.basePipelineIndex = -1;

    VkResult result = vkCreateComputePipelines(*p_device, VK_NULL_HANDLE, 1, &comp_pipeline_create_info, nullptr, &pipeline);
}