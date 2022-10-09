#pragma once
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <memory>

#include "vulkan/vulkan.h"
#define VMA_IMPLEMENTATION
#define VMA_VULKAN_VERSION 1002000
#include "vk_mem_alloc.h"

#include <iostream>
#include <vector>

#ifdef __unix__
    #include <unistd.h>
    #include <libgen.h>
    #define GETCWD(x, size) getcwd(x, size)
#elif defined(_WIN32)
    #include <direct.h>
    #define GETCWD(x, size) _getcwd(x, size)
#endif

#include <string.h>
#include <fstream>

#define VK_ASSERT(x) if(x != VK_SUCCESS) { printf("[vk-cholesky] vk runtime error %x\n", x); assert(x == VK_SUCCESS); }

namespace vx {

// 
// wrapping crer
// clear to view
// call function dependency
// 
struct Buffer {
    VkBuffer            self;
    VmaAllocation       allocation;
    VmaAllocationInfo   info;

    void copy(void* data, size_t size) {

        assert( data!=nullptr );

        // default
        memcpy(info.pMappedData, data, size);
    }
};

struct CommandBuffer {
    VkCommandBuffer self;

    inline void begin() {
        VkCommandBufferBeginInfo info = {};
        info.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;

        VK_ASSERT(vkBeginCommandBuffer(self, &info));
    };

    inline void end() {
        VK_ASSERT(vkEndCommandBuffer(self));
    }

    inline void copyBuffer(VkBuffer src, VkBuffer dst, VkBufferCopy copy) {
        vkCmdCopyBuffer(self, src, dst, 1, &copy);
    };
};

struct Device {

    VkDevice        self;
    VkCommandPool   commandPool;
    VmaAllocator    allocator;
    VkQueue         queue;

    inline Buffer createBuffer(VkBufferCreateInfo buf_info, VmaAllocationCreateInfo alloc_info) const {
        Buffer buf;
        
        //@@ error checking
        VK_ASSERT(vmaCreateBuffer( allocator, &buf_info, &alloc_info, &buf.self, &buf.allocation, &buf.info ));
        
        return buf;
    };

    inline CommandBuffer allocateCommandBuffer() const {
        CommandBuffer cmdBuffer;

        VkCommandBufferAllocateInfo info = {};
        info.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
        info.commandPool = commandPool;
        info.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
        info.commandBufferCount = 1;

        //@@ error checking
        VK_ASSERT(vkAllocateCommandBuffers(self, &info, &cmdBuffer.self));

        return cmdBuffer;
    }

    inline void freeCommandBuffers(const VkCommandBuffer* cmd, size_t size) const {
        vkFreeCommandBuffers(self, commandPool, size, cmd);
    }
};

struct vkcontext_t {
    vkcontext_t();

    VkInstance instance; // VK_NULL_HANDLE
    VkPhysicalDevice physical_device;
    uint32_t queue_family_index;

    Device device;    

private:
    void create_instance();
    void create_physical_device();
    void initVxdevice();
};

vkcontext_t::vkcontext_t() {
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
    VK_ASSERT(vkCreateInstance(&instance_create_info, nullptr, &instance));

    // debugger

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

    initVxdevice();
}

void vkcontext_t::initVxdevice() {
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
    VK_ASSERT(vkCreateDevice(physical_device, &device_create_info, nullptr, &device.self));

    //
    // get queues
    //
    vkGetDeviceQueue( device.self, queue_family_index, 0, &device.queue );

    //
    // vk command pool
    //
    VkCommandPoolCreateInfo cmd_pool_create_info ={};
    cmd_pool_create_info.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
    cmd_pool_create_info.queueFamilyIndex = queue_family_index;
    cmd_pool_create_info.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
    VK_ASSERT(vkCreateCommandPool(device.self, &cmd_pool_create_info, nullptr, &device.commandPool));

    // vma allocator
    VmaAllocatorCreateInfo allocator_cinfo = {};
    allocator_cinfo.instance = instance;
    allocator_cinfo.physicalDevice = physical_device;
    allocator_cinfo.device = device.self;
    VK_ASSERT(vmaCreateAllocator(&allocator_cinfo, &device.allocator));
}

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

static std::vector<char> readFile( const std::string& filename ) {
    std::ifstream file( filename, std::ios::ate | std::ios::binary );

    if( !file.is_open() ) {
        throw std::runtime_error( "failed to open file!" );
    }

    size_t fileSize = (size_t) file.tellg();
    std::vector<char> buffer(fileSize);

    file.seekg(0);
    file.read(buffer.data(), fileSize);
    file.close();

    return buffer;
}

struct Pipeline {

    Pipeline( const Device* device );

// func
    // shaders
    void create_specialization();
    VkShaderModule create_shader_module();
    
// member
    VkPipeline self;
    const Device* p_device; // borrowing
};

VkShaderModule Pipeline::create_shader_module() {

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

    // sprintf( path, "/home/hyeonjang/vk-cholesky/src/shader/cholesky.spv", __FILE__ );
    auto code = readFile( path );
    VkShaderModuleCreateInfo info{};
    info.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    // info.codeSize = read_file_length(path);
    // info.pCode = read_file(path, info.codeSize);
    info.codeSize = code.size();
    info.pCode = reinterpret_cast<uint32_t*>(code.data());

    VkShaderModule shadermodule;
    VK_ASSERT(vkCreateShaderModule(p_device->self, &info, nullptr, &shadermodule));
    
    return shadermodule;
}

Pipeline::Pipeline(const Device* device)
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
    pool_size.type = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    pool_size.descriptorCount = 1;

    VkDescriptorPoolCreateInfo desc_pool_create_info = {};
    desc_pool_create_info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    desc_pool_create_info.poolSizeCount = 1;
    desc_pool_create_info.pPoolSizes = &pool_size;
    desc_pool_create_info.maxSets = 1;
    VK_ASSERT(vkCreateDescriptorPool(p_device->self, &desc_pool_create_info, nullptr, &desc_pool));
    
    // desc layout
    VkDescriptorSet desc_set;
    VkDescriptorSetLayout desc_set_layout;
    VkDescriptorSetLayoutBinding desc_set_layout_binding = {};
    desc_set_layout_binding.descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    desc_set_layout_binding.stageFlags = VK_SHADER_STAGE_COMPUTE_BIT;
    desc_set_layout_binding.binding = 0;
    desc_set_layout_binding.descriptorCount = 1;

    VkDescriptorSetLayoutCreateInfo desc_set_layout_cinfo = {};
    desc_set_layout_cinfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    desc_set_layout_cinfo.bindingCount = 1;
    desc_set_layout_cinfo.pBindings = &desc_set_layout_binding;
    VK_ASSERT(vkCreateDescriptorSetLayout(p_device->self, &desc_set_layout_cinfo, nullptr, &desc_set_layout));

        // pipeline layout
    VkPipelineLayout pipeline_layout;
    VkPipelineLayoutCreateInfo layout_info = {};
    layout_info.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    layout_info.setLayoutCount = 1;
    layout_info.pSetLayouts = &desc_set_layout;
    layout_info.pushConstantRangeCount = 0;
    layout_info.pPushConstantRanges = nullptr;
    VK_ASSERT(vkCreatePipelineLayout(p_device->self, &layout_info, nullptr, &pipeline_layout));

    VkDescriptorSetAllocateInfo desc_set_alloc_info = {};
    desc_set_alloc_info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    desc_set_alloc_info.descriptorPool = desc_pool;
    desc_set_alloc_info.pSetLayouts = &desc_set_layout;
    desc_set_alloc_info.descriptorSetCount = 1;
    VK_ASSERT(vkAllocateDescriptorSets(p_device->self, &desc_set_alloc_info, &desc_set));

    //
    // buffers
    //
    VkBufferCreateInfo buffer_info = {};
    buffer_info.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    buffer_info.size = 65536;
    buffer_info.usage = VK_BUFFER_USAGE_STORAGE_BUFFER_BIT;
    
    VmaAllocationCreateInfo alloc_info = {};
    alloc_info.usage = VMA_MEMORY_USAGE_CPU_TO_GPU;
    alloc_info.flags = VMA_ALLOCATION_CREATE_MAPPED_BIT;
    Buffer buffer = p_device->createBuffer(buffer_info, alloc_info);

    size_t size = 32;
    std::vector<uint32_t> computeInput(size);
    std::vector<uint32_t> computeOutput(size);
    uint32_t n = 0;
    std::generate( computeInput.begin(), computeInput.end(), [&n]{ return n++;  } );
    buffer.copy((void*)&computeInput[0], sizeof(uint32_t)*size);

    std::cout << computeInput[3] << std::endl;

    //
    // command buffers
    //
    CommandBuffer cmdBuffer = p_device->allocateCommandBuffer();
    cmdBuffer.begin();
    cmdBuffer.end();

    VkFence fence;
    VkFenceCreateInfo fenceInfo ={};
    fenceInfo.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
    // fenceInfo.flags = VK_FLAGS_NONE;
    VK_ASSERT(vkCreateFence(p_device->self, &fenceInfo, nullptr, &fence));

    VkSubmitInfo submitInfo ={};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;

    p_device->freeCommandBuffers(&cmdBuffer.self, 1);

    VkDescriptorBufferInfo desc_buffer_info = {};
    desc_buffer_info.buffer = buffer.self;
    desc_buffer_info.offset = 0;
    desc_buffer_info.range = VK_WHOLE_SIZE;
    VkWriteDescriptorSet write_desc_set = {};
    write_desc_set.sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
    write_desc_set.descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
    write_desc_set.dstSet = desc_set;
    write_desc_set.pBufferInfo = &desc_buffer_info;
    write_desc_set.pImageInfo = nullptr;
    write_desc_set.descriptorCount = 1;
    write_desc_set.dstBinding = 0;
    vkUpdateDescriptorSets(p_device->self, 1, &write_desc_set, 0, NULL);

    printf( "descriptor done\n" );


    VkComputePipelineCreateInfo comp_pipeline_create_info = {};
    comp_pipeline_create_info.sType = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO;
    comp_pipeline_create_info.stage = comp_shader_info;
    comp_pipeline_create_info.layout = pipeline_layout;
    comp_pipeline_create_info.basePipelineHandle = VK_NULL_HANDLE;
    comp_pipeline_create_info.basePipelineIndex = 0;

    VkPipelineCacheCreateInfo pipeline_cache_create_info = {};
    pipeline_cache_create_info.sType = VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO;
    VkPipelineCache pipeline_cache;
    vkCreatePipelineCache(p_device->self, &pipeline_cache_create_info, nullptr, &pipeline_cache);

    printf("pipelinecreation\n");
    VkPipeline compute_pipeline;
    VK_ASSERT(vkCreateComputePipelines(p_device->self, VK_NULL_HANDLE, 1, &comp_pipeline_create_info, nullptr, &compute_pipeline));
}
}