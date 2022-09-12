#pragma once
#include <stdio.h>
#include <stdlib.h>
#include "vulkan/vulkan.h"

struct vkcontext_t {
    VkInstance instance; // VK_NULL_HANDLE
    VkPhysicalDevice physical_device;

    vkcontext_t();
    VkDevice get_device();
    void create_device();    
};

vkcontext_t::vkcontext_t() {
    VkApplicationInfo app_info;
    app_info.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    app_info.pApplicationName = "vk-cholesky";
    app_info.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    app_info.pEngineName = "No Engine";
    app_info.engineVersion = VK_API_VERSION_1_0;
    printf("create instance\n");

    VkInstanceCreateInfo instance_create_info;
    instance_create_info.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    instance_create_info.pApplicationInfo = &app_info;
    instance_create_info.enabledLayerCount = 0;
    instance_create_info.enabledExtensionCount = 0;

    VkResult result = vkCreateInstance(&instance_create_info, NULL, &instance);

    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);

    VkPhysicalDevice* physical_devices = (VkPhysicalDevice*)malloc(sizeof(VkPhysicalDevice)*deviceCount);
    vkEnumeratePhysicalDevices(instance, &deviceCount, physical_devices);

    physical_device = physical_devices[0];
}

void vkcontext_t::create_device() {

    VkDeviceQueueCreateInfo device_queue_create_info;

    VkDeviceCreateInfo device_create_info;

}