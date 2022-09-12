#pragma once
#include <stdio.h>
#include <stdlib.h>
#include <memory>
#include "vulkan/vulkan.h"

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

struct vkcontext_t {
    vkcontext_t();
    std::unique_ptr<VkDevice> get_device();

    VkInstance instance; // VK_NULL_HANDLE
    VkPhysicalDevice physical_device;
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
    
    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);
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
}

std::unique_ptr<VkDevice> vkcontext_t::get_device() {

    VkDeviceQueueCreateInfo device_queue_create_info;

    VkDeviceCreateInfo device_create_info;
    device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    device_create_info.pQueueCreateInfos = &device_queue_create_info;

    VkDevice* p_device = nullptr;
    VkResult result = vkCreateDevice(physical_device, &device_create_info, nullptr, p_device);

    return std::unique_ptr<VkDevice>(p_device);
}

struct vk_compute_pipeline_t {

    vk_compute_pipeline_t(VkDevice* device);

    VkPipeline pipeline;
};

vk_compute_pipeline_t::vk_compute_pipeline_t(VkDevice* device) {



}