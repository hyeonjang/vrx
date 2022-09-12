#pragma once
#include <stdio.h>
#include "vulkan/vulkan.h"

struct vkcontext_t {
    VkInstance instance;
    VkPhysicalDevice physical_device;
};

void vk_create_context(vkcontext_t* vkcontext) {
    printf("create instance\n");
 
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
    
    printf("create instance %p\n", (void*)&vkcontext->instance);
    VkResult result = vkCreateInstance(&instance_create_info, nullptr, &vkcontext->instance);
    printf("create instance %p\n", (void*)&vkcontext->instance);
}