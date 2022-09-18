#pragma once
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <memory>

#include "vulkan/vulkan.h"
#define VMA_IMPLEMENTATION
#include "vk_mem_alloc.h"

#include <iostream>
#include <vector>

#define VK_ASSERT(x) if(x != VK_SUCCESS) { printf("[vk-cholesky] vk runtime error %x\n", x); assert(x == VK_SUCCESS); }

struct vkcontext_t {
    vkcontext_t();
    VkDevice get_device();

private:
    void create_instance();
    void create_physical_device();
    void create_device();

    VkInstance instance; // VK_NULL_HANDLE
    VkPhysicalDevice physical_device;
    uint32_t queue_family_index;
    VkDevice device;
    VmaAllocator allocator;
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
    VK_ASSERT(vkCreateInstance(&instance_create_info, nullptr, &instance));

    // physcial device
    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);

    std::vector<VkPhysicalDevice> physical_devices(deviceCount);
    VK_ASSERT(vkEnumeratePhysicalDevices(instance, &deviceCount, &physical_devices[0]));
    physical_device = physical_devices[0];

    // queue family index
    uint32_t queue_family_count = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, nullptr);

    std::vector<VkQueueFamilyProperties> queue_family_properties(queue_family_count);
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &queue_family_count, &queue_family_properties[0]);

    for(uint32_t i=0; i<queue_family_count; i++) {
        if(queue_family_properties[i].queueFlags & VK_QUEUE_COMPUTE_BIT) {
        // if(queue_family_properties[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) {
            queue_family_index = i;
            // printf( "queue family index: %d", queue_family_index );
        }
    }
    //queue_family_index = 0;
    printf( "queue family index: %d\n", queue_family_index );

    // vma allocator
    VmaAllocatorCreateInfo allocator_cinfo = {};
    allocator_cinfo.instance = instance;
    allocator_cinfo.physicalDevice = physical_device;
    allocator_cinfo.device = device;
    VK_ASSERT(vmaCreateAllocator(&allocator_cinfo, &allocator));
}

//std::unique_ptr<VkDevice> vkcontext_t::get_device() {
VkDevice vkcontext_t::get_device() {

    float queue_priority = 1.0;

    VkDeviceQueueCreateInfo device_queue_create_info {};
    device_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    device_queue_create_info.queueFamilyIndex = queue_family_index;
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

    //VkDevice* p_device = new VkDevice;
    VkDevice device;
    VK_ASSERT(vkCreateDevice(physical_device, &device_create_info, nullptr, &device));

    return device;
}



#ifdef __unix__
    #include <unistd.h>
    #include <libgen.h>
    #define GETCWD(x, size) getcwd(x, size)
#elif defined(_WIN32)
    #include <direct.h>
    #define GETCWD(x, size) _getcwd(x, size)
#endif

#include <string.h>

static size_t read_file_length(const char* filepath) {

    char cwd[1024];
    GETCWD( cwd, sizeof( cwd ) );

    FILE* fp = fopen( filepath, "r" );
    assert( fp!=NULL );

    int seek_result = fseek( fp, 0, SEEK_END );
    size_t size = ftell(fp);
    
    fclose( fp );
    return size;
}

static uint32_t* read_file(const char* filepath, size_t size) {

    char* buffer = (char*) malloc(sizeof(char)*size);
    FILE* fp = fopen(filepath, "r");

    fgets( buffer, sizeof( buffer ), fp );

    fclose(fp);
    return reinterpret_cast<uint32_t*>(buffer);
}

struct vk_pipeline_t {

    vk_pipeline_t(VkDevice* p_device);

// func
    // shaders
    void create_specialization();
    VkShaderModule create_shader_module();
    
    // pipelines
    // somethings

// member
    VkPipeline pipeline;
    const VkDevice* p_device;
};

VkShaderModule vk_pipeline_t::create_shader_module() {

    // call glslc compiler

    // std::cout << sizeof(__FILE__) << std::endl;

    // const char* file = __FILE__;
    // const char* check = strstr(file, "/vkcontext.h");
    char path[80];
    sprintf( path, "%s/../shader/cholesky.spv", __FILE__ );
    // char* tocheck = "home,hyeonjang,vk,cholesky\0";
    // char *token, *string = "a string, of, ,tokens\0,after null terminator";
    // token = strtok(string, ",");
    // printf("token: %s\n", token);
    // do {
    //     printf("token: %s\n", token);
    // } while (token = strtok(NULL, "/"));

    //sprintf( path, "/home/hyeonjang/vk-cholesky/src/shader/cholesky.spv", __FILE__ );

    VkShaderModuleCreateInfo info{};
    info.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    info.codeSize = read_file_length(path);
    info.pCode = read_file(path, info.codeSize);

    VkShaderModule shadermodule;
    VK_ASSERT(vkCreateShaderModule(*p_device, &info, nullptr, &shadermodule));
    
    return shadermodule;
}

vk_pipeline_t::vk_pipeline_t(VkDevice* device)
:p_device(device) {

    struct specialization_t {
        uint32_t BUFFER_ELEMENT_COUNT = 32;
    } speicalization;

    VkSpecializationMapEntry spec_map_entry;
    spec_map_entry.constantID = 0;
    spec_map_entry.offset = 0;
    spec_map_entry.size = sizeof( uint32_t );

    VkSpecializationInfo spec_info;
    spec_info.mapEntryCount = 1;
    spec_info.pMapEntries = &spec_map_entry;
    spec_info.dataSize = sizeof(specialization_t);
    spec_info.pData = (void*)(&speicalization);

    VkPipelineShaderStageCreateInfo comp_shader_info = {};
    comp_shader_info.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    comp_shader_info.stage = VK_SHADER_STAGE_COMPUTE_BIT;
    comp_shader_info.module = create_shader_module();
    comp_shader_info.pSpecializationInfo = &spec_info;
    comp_shader_info.pName = "main";
    assert(comp_shader_info.module != VK_NULL_HANDLE);

    // desc pool
    VkDescriptorPool desc_pool;
    VkDescriptorPoolSize pool_size ={};
    VkDescriptorPoolCreateInfo desc_pool_create_info = {};
    desc_pool_create_info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    desc_pool_create_info.poolSizeCount = 1;
    desc_pool_create_info.pPoolSizes = &pool_size;
    desc_pool_create_info.maxSets = 1;
    VK_ASSERT(vkCreateDescriptorPool(*p_device, &desc_pool_create_info, nullptr, &desc_pool));
    
    // desc layout
    VkDescriptorSet desc_set;
    VkDescriptorSetLayout desc_set_layout;
    VkDescriptorSetLayoutBinding desc_set_layout_binding = {};
    desc_set_layout_binding.descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    desc_set_layout_binding.stageFlags = VK_SHADER_STAGE_COMPUTE_BIT;
    desc_set_layout_binding.descriptorCount = 0;

    VkDescriptorSetLayoutCreateInfo desc_set_layout_cinfo = {};
    desc_set_layout_cinfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    desc_set_layout_cinfo.bindingCount = 1;
    desc_set_layout_cinfo.pBindings = &desc_set_layout_binding;
    VK_ASSERT(vkCreateDescriptorSetLayout(*p_device, &desc_set_layout_cinfo, nullptr, &desc_set_layout));

    VkDescriptorSetAllocateInfo desc_set_alloc_info = {};
    desc_set_alloc_info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    desc_set_alloc_info.descriptorPool = desc_pool;
    desc_set_alloc_info.descriptorSetCount = 1;
    desc_set_alloc_info.pSetLayouts = &desc_set_layout;
    VK_ASSERT(vkAllocateDescriptorSets(*p_device, &desc_set_alloc_info, &desc_set));

    VkBuffer deviceBuffer;
    {

    }

    VkDescriptorBufferInfo desc_buffer_info = {};
    desc_buffer_info.buffer = deviceBuffer;
    VkWriteDescriptorSet write_desc_set = {};
    write_desc_set.sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    write_desc_set.descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    write_desc_set.dstSet = desc_set;
    write_desc_set.pBufferInfo = &desc_buffer_info;
    vkUpdateDescriptorSets(*p_device, 1, &write_desc_set, 0, NULL);

    printf( "descriptor done\n" );

    // pipeline layout
    VkPipelineLayout pipeline_layout;
    VkPipelineLayoutCreateInfo layout_info = {};
    layout_info.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    layout_info.setLayoutCount = 1;
    layout_info.pSetLayouts = &desc_set_layout;
    layout_info.pushConstantRangeCount = 0;
    layout_info.pPushConstantRanges = nullptr;
    VK_ASSERT(vkCreatePipelineLayout(*p_device, &layout_info, nullptr, &pipeline_layout));

    VkComputePipelineCreateInfo comp_pipeline_create_info = {};
    comp_pipeline_create_info.sType = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO;
    comp_pipeline_create_info.stage = comp_shader_info;
    comp_pipeline_create_info.layout = pipeline_layout;
    comp_pipeline_create_info.basePipelineHandle = VK_NULL_HANDLE;
    comp_pipeline_create_info.basePipelineIndex = 0;

    VkPipelineCacheCreateInfo pipeline_cache_create_info = {};
    pipeline_cache_create_info.sType = VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO;
    VkPipelineCache pipeline_cache;
    vkCreatePipelineCache(*p_device, &pipeline_cache_create_info, nullptr, &pipeline_cache);

    printf("pipelinecreation\n");
    VkPipeline compute_pipeline;
    VK_ASSERT(vkCreateComputePipelines(*p_device, VK_NULL_HANDLE, 1, &comp_pipeline_create_info, nullptr, &compute_pipeline));
}