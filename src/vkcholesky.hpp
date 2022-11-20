#pragma once

#include <stdio.h>
#include <string.h>

#include <vulkan/vulkan.h>

// // #include <iostream>
// // #include <vector>

// // #ifdef __unix__
// //     #include <unistd.h>
// //     #include <libgen.h>
// //     #define GETCWD(x, size) getcwd(x, size)
// // #elif defined(_WIN32)
// //     #include <direct.h>
// //     #define GETCWD(x, size) _getcwd(x, size)
// // #endif

// // #include <string.h>
// // #include <fstream>

// // #define VK_ASSERT(x) if(x != VK_SUCCESS) { printf("[vk-cholesky] vk runtime error %x\n", x); assert(x == VK_SUCCESS); }

//  
// wrapping crer
// clear to view
// call function dependency
// 
static VkInstance       g_instance;
static VkPhysicalDevice g_physicalDevice;
extern VkDevice         g_device;
static uint32_t         g_queueFamillyIndex;
static VkQueue          g_queue;
static VkCommandPool    g_commandPool;

void initVulkan();

struct Buffer {
    VkBuffer            self;
private:
    VkDeviceMemory      memory;
    void*               data;
    size_t              size;
public:
    Buffer(VkBufferCreateInfo info, const VkDevice* device);
    void alloc(VkMemoryAllocateInfo info);
    void map(void* _data);
};

struct CommandBuffer {
    VkCommandBuffer self;

    void begin();
    void end();

    void copyBuffer(VkBuffer src, VkBuffer dst, uint32_t regionCount, const VkBufferCopy* copy);
    void bindPipeline(VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline);

    inline void bindDescriptorSets(
        VkPipelineBindPoint     pipelineBindPoint,
        VkPipelineLayout        pipelineLayout,
        uint32_t                firstSet,
        uint32_t                descriptorSetCount,
        const VkDescriptorSet*  pDescriptorSet,
        uint32_t                dynamicOffsetCount,
        const uint32_t*         pDynamicOffsets
        ) {
        vkCmdBindDescriptorSets(
            self, pipelineBindPoint, 
            pipelineLayout, firstSet, 
            descriptorSetCount, pDescriptorSet, 
            dynamicOffsetCount, pDynamicOffsets
        );
    }

    inline void dispatch(uint32_t countX, uint32_t countY, uint32_t countZ) {
        vkCmdDispatch( self, countX, countY, countZ );
    }

    inline void pipelineBarrier(
        VkPipelineStageFlags            flag0, 
        VkPipelineStageFlags            flag1, 
        VkDependencyFlags               dependencyFlag, 
        uint32_t                        nMemomryBarrier, 
        const VkMemoryBarrier*          pMemomryBarrier,
        uint32_t                        nBufferMemomryBarrier, 
        const VkBufferMemoryBarrier*    pBufferMemomryBarrier,
        uint32_t                        nImageMemoryBarrier,
        const VkImageMemoryBarrier*     pImageMemoryBarrier) {
        vkCmdPipelineBarrier(
            self, flag0, flag1, dependencyFlag, 
            nMemomryBarrier, pMemomryBarrier, 
            nBufferMemomryBarrier, pBufferMemomryBarrier, 
            nImageMemoryBarrier, pImageMemoryBarrier
        );
    }
    
    inline void pipelineBarrier(
        VkPipelineStageFlags            flag0, 
        VkPipelineStageFlags            flag1, 
        VkDependencyFlags               dependencyFlag, 
        uint32_t                        nBufferMemomryBarrier, 
        const VkBufferMemoryBarrier*    pBufferMemomryBarrier) {
        vkCmdPipelineBarrier(
            self, flag0, flag1, dependencyFlag, 
            0, nullptr, 
            nBufferMemomryBarrier, pBufferMemomryBarrier, 
            0, nullptr
        );
    }
};

struct Descriptor {

    Descriptor();
    void updateDescriptorSets();
private:
    void create_descriptor_pool();
    void create_descriptor_layout();
    void allocate_descriptor_set();
public:
    VkDescriptorPool         pool;
    VkDescriptorSet*         sets;
    VkDescriptorSetLayout*   setLayouts;
    uint32_t                 count;
};

// static size_t read_file_length(const char* filepath) {

//     char cwd[1024];
//     GETCWD( cwd, sizeof( cwd ) );

//     FILE* fp = fopen( filepath, "r" );
//     assert( fp!=NULL );

//     int seek_result = fseek( fp, 0, SEEK_END );
//     size_t size = ftell(fp);
    
//     fclose( fp );
//     return size;
// }

// static uint32_t* read_file(const char* filepath, size_t size) {

//     char* buffer = (char*) malloc(sizeof(char)*size);
//     FILE* fp = fopen(filepath, "r");

//     fgets( buffer, sizeof( buffer ), fp );

//     fclose(fp);
//     return reinterpret_cast<uint32_t*>(buffer);
// }

// VkShaderModule create_shader_module(const uint32_t* shader_code_data, size_t size);

struct ComputePipeline {


public:
    VkPipeline pipeline;
};

// struct Pipeline {

//     Pipeline( const Device* device );

// // func
//     // shaders
//     void create_specialization();
//     void compile_shader();
//     VkShaderModule   create_shader_module();
//     VkPipelineLayout create_pipeline_layout(VkDescriptorSetLayout* descSetLayout, size_t layoutSize);
    
// // member
//     VkPipeline      self;
//     const Device*   p_device; // borrowing
// };

// void Pipeline::compile_shader() {

//     char command[256];
//     sprintf( command, "glslc %s/../shader/cholesky.comp -o %s/../shader/cholesky.spv", __FILE__, __FILE__ );
//     system( command );
// }

// VkShaderModule Pipeline::create_shader_module() {

//     // call glslc compiler

//     // std::cout << sizeof(__FILE__) << std::endl;
//     compile_shader();
//     // const char* file = __FILE__;
//     // const char* check = strstr(file, "/vkcontext.h");
//     char path[80];
//     sprintf( path, "%s/../shader/cholesky.spv", __FILE__ );
//     // char* tocheck = "home,hyeonjang,vk,cholesky\0";
//     // char *token, *string = "a string, of, ,tokens\0,after null terminator";
//     // token = strtok(string, ",");
//     // printf("token: %s\n", token);
//     // do {
//     //     printf("token: %s\n", token);
//     // } while (token = strtok(NULL, "/"));

//     //sprintf( path, "/home/hyeonjang/vk-cholesky/src/shader/cholesky.spv", __FILE__ );
//     auto code = readFile( path );
//     VkShaderModuleCreateInfo info{};
//     info.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
//     // info.codeSize = read_file_length(path);
//     // info.pCode = read_file(path, info.codeSize);
//     info.codeSize = code.size();
//     info.pCode = reinterpret_cast<uint32_t*>(code.data());

//     VkShaderModule shadermodule;
//     VK_ASSERT(vkCreateShaderModule(p_device->self, &info, nullptr, &shadermodule));
    
//     return shadermodule;
// }

// VkPipelineLayout Pipeline::create_pipeline_layout(VkDescriptorSetLayout* descSetLayout, size_t layoutSize) {

//     VkPipelineLayout layout;

//     VkPipelineLayoutCreateInfo info ={};
//     info.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
//     info.setLayoutCount = layoutSize;
//     info.pSetLayouts = descSetLayout;
//     info.pushConstantRangeCount = 0;
//     info.pPushConstantRanges = nullptr;
    
//     VK_ASSERT(vkCreatePipelineLayout(p_device->self, &info, nullptr, &layout));

//     return layout;
// }

// Pipeline::Pipeline(const Device* device)
// :p_device(device) {

//     // descriptor
//     Descriptor desc( &p_device->self );

//     //
//     // buffers
//     //
//     VkBufferCreateInfo hostBufInfo = {};
//     hostBufInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
//     hostBufInfo.size = 65536;
//     hostBufInfo.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT;
//     VmaAllocationCreateInfo hostAllocInfo = {};
//     hostAllocInfo.usage = VMA_MEMORY_USAGE_GPU_TO_CPU;
//     hostAllocInfo.flags = VMA_ALLOCATION_CREATE_MAPPED_BIT;
//     Buffer bufHost = p_device->createBuffer(hostBufInfo, hostAllocInfo);

//     size_t size = 32;
//     std::vector<uint32_t> computeInput(size);
//     std::vector<uint32_t> computeOutput(size);
//     uint32_t n = 0;
//     std::generate( computeInput.begin(), computeInput.end(), [&n]{ return n++;  } );
//     bufHost.copy((void*)&computeInput[0], sizeof(uint32_t)*size);
//     std::cout << computeInput[2] << std::endl;

//     VkBufferCreateInfo deviceBufInfo = {};
//     deviceBufInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
//     deviceBufInfo.size = 65536;
//     deviceBufInfo.usage = VK_BUFFER_USAGE_STORAGE_BUFFER_BIT | VK_BUFFER_USAGE_TRANSFER_SRC_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT;
//     VmaAllocationCreateInfo deviceAllocInfo = {};
//     deviceAllocInfo.usage = VMA_MEMORY_USAGE_CPU_TO_GPU;
//     deviceAllocInfo.flags = VMA_ALLOCATION_CREATE_MAPPED_BIT;
//     Buffer bufDevice = p_device->createBuffer(deviceBufInfo, deviceAllocInfo);
    
//     CommandBuffer cmdBuffer = p_device->allocateCommandBuffer();
//     {
//         cmdBuffer.begin();
//         VkBufferCopy copyRegion ={};
//         copyRegion.size = sizeof( uint32_t )*size;
//         cmdBuffer.copyBuffer( bufHost.self, bufDevice.self, 1, &copyRegion );
//         cmdBuffer.end();
//     }

//     VkSubmitInfo submitInfo ={};
//     submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
//     submitInfo.commandBufferCount = 1;
//     submitInfo.pCommandBuffers = &cmdBuffer.self;
//     VkFence fence;
//     VkFenceCreateInfo fenceInfo ={};
//     fenceInfo.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
//     fenceInfo.flags = 0;
//     VK_ASSERT(vkCreateFence(p_device->self, &fenceInfo, nullptr, &fence));

//     p_device->queueSubmit(submitInfo, fence);
//     vkWaitForFences(p_device->self, 1, &fence, VK_TRUE, UINT64_MAX);

//     vkDestroyFence(p_device->self, fence, nullptr);
//     p_device->freeCommandBuffers(&cmdBuffer.self, 1);

//     VkDescriptorBufferInfo desc_buffer_info = {};
//     desc_buffer_info.buffer = bufDevice.self;
//     desc_buffer_info.offset = 0;
//     desc_buffer_info.range = VK_WHOLE_SIZE;
//     VkWriteDescriptorSet write_desc_set = {};
//     write_desc_set.sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
//     write_desc_set.descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER;
//     write_desc_set.dstSet = desc.set;
//     write_desc_set.pBufferInfo = &desc_buffer_info;
//     write_desc_set.pImageInfo = nullptr;
//     write_desc_set.descriptorCount = 1;
//     write_desc_set.dstBinding = 0;
//     vkUpdateDescriptorSets(p_device->self, 1, &write_desc_set, 0, NULL);

//     printf( "descriptor done\n" );
//     struct specialization_t {
//         uint32_t BUFFER_ELEMENT_COUNT = 32;
//     } speicalization;

//     VkSpecializationMapEntry spec_map_entry;
//     spec_map_entry.constantID = 0;
//     spec_map_entry.offset = 0;
//     spec_map_entry.size = sizeof( uint32_t );

//     VkSpecializationInfo spec_info;
//     spec_info.mapEntryCount = 1;
//     spec_info.pMapEntries = &spec_map_entry;
//     spec_info.dataSize = sizeof(speicalization);
//     spec_info.pData = (void*)(&speicalization);

//     VkPipelineShaderStageCreateInfo comp_shader_info = {};
//     comp_shader_info.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
//     comp_shader_info.stage = VK_SHADER_STAGE_COMPUTE_BIT;
//     comp_shader_info.module = create_shader_module();
//     comp_shader_info.pSpecializationInfo = &spec_info;
//     comp_shader_info.pSpecializationInfo = &spec_info;
//     comp_shader_info.pName = "main";
//     assert(comp_shader_info.module != VK_NULL_HANDLE);

//     VkPipelineLayout pipelineLayout = create_pipeline_layout(&desc.setLayout, 1);
//     VkComputePipelineCreateInfo comp_pipeline_create_info = {};
//     comp_pipeline_create_info.sType = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO;
//     comp_pipeline_create_info.stage = comp_shader_info;
//     comp_pipeline_create_info.layout = pipelineLayout;
//     comp_pipeline_create_info.basePipelineHandle = VK_NULL_HANDLE;
//     comp_pipeline_create_info.basePipelineIndex = 0;

//     VkPipelineCacheCreateInfo pipeline_cache_create_info = {};
//     pipeline_cache_create_info.sType = VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO;
//     VkPipelineCache pipeline_cache;
//     vkCreatePipelineCache(p_device->self, &pipeline_cache_create_info, nullptr, &pipeline_cache);

//     printf("pipelinecreation\n");

//     VkPipeline computePipeline;
//     VK_ASSERT(vkCreateComputePipelines(p_device->self, VK_NULL_HANDLE, 1, &comp_pipeline_create_info, nullptr, &computePipeline));


//     //@@ add command buffer submit for ss

//     //
//     CommandBuffer commandBuffer = p_device->allocateCommandBuffer();
//     VkFenceCreateInfo fenceCreateInfo = {};
//     fenceCreateInfo.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
//     fenceCreateInfo.flags = VK_FENCE_CREATE_SIGNALED_BIT;
//     VK_ASSERT(vkCreateFence(p_device->self, &fenceCreateInfo, nullptr, &fence));

//     commandBuffer.begin();
//     VkBufferMemoryBarrier bufferBarrier0 = {};
//     bufferBarrier0.sType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER;
//     bufferBarrier0.srcAccessMask = VK_ACCESS_HOST_WRITE_BIT;
//     bufferBarrier0.dstAccessMask = VK_ACCESS_SHADER_READ_BIT;
//     bufferBarrier0.buffer = bufDevice.self;
//     bufferBarrier0.size = VK_WHOLE_SIZE;
//     bufferBarrier0.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
//     bufferBarrier0.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;

//     commandBuffer.pipelineBarrier(
//         VK_PIPELINE_STAGE_HOST_BIT, 
//         VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT, 
//         0, 1, &bufferBarrier0
//         );
//     commandBuffer.bindPipeline(VK_PIPELINE_BIND_POINT_COMPUTE, computePipeline);

//     commandBuffer.bindDescriptorSets(
//         VK_PIPELINE_BIND_POINT_COMPUTE,
//         pipelineLayout,
//         0, 1, &desc.set,
//         0, 0
//     );

//     commandBuffer.dispatch(
//         32, 1, 1
//     );

//     //VkBufferMemoryBarrier bufferBarrier1 = {};
//     //bufferBarrier0.sType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER;
//     bufferBarrier0.srcAccessMask = VK_ACCESS_SHADER_WRITE_BIT;
//     bufferBarrier0.dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
//     bufferBarrier0.buffer = bufDevice.self;
//     bufferBarrier0.size = VK_WHOLE_SIZE;
//     bufferBarrier0.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
//     bufferBarrier0.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;

//     commandBuffer.pipelineBarrier(
//         VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT,
//         VK_PIPELINE_STAGE_TRANSFER_BIT,
//         0, 1, &bufferBarrier0
//     );

//     //VkBufferMemoryBarrier bufferBarrier2 = {};
//     //bufferBarrier0.sType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER;
//     bufferBarrier0.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
//     bufferBarrier0.dstAccessMask = VK_ACCESS_HOST_READ_BIT;
//     bufferBarrier0.buffer = bufHost.self;
//     bufferBarrier0.size = VK_WHOLE_SIZE;
//     bufferBarrier0.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
//     bufferBarrier0.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;

//     commandBuffer.pipelineBarrier(
//         VK_PIPELINE_STAGE_TRANSFER_BIT,
//         VK_PIPELINE_STAGE_HOST_BIT,
//         0, 1, &bufferBarrier0
//     );

//     commandBuffer.end();

//     vkResetFences( p_device->self, 1, &fence );

//     const VkPipelineStageFlags waitStageMask = VK_PIPELINE_STAGE_TRANSFER_BIT;
//     VkSubmitInfo computeSubmitInfo ={};
//     computeSubmitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
//     computeSubmitInfo.pWaitDstStageMask = &waitStageMask;
//     computeSubmitInfo.commandBufferCount = 1;
//     computeSubmitInfo.pCommandBuffers = &commandBuffer.self;
//     vkQueueSubmit( p_device->queue, 1, &computeSubmitInfo, fence );
//     vkWaitForFences( p_device->self, 1, &fence, VK_TRUE, UINT64_MAX);

//     std::vector<uint32_t> input( bufDevice.size(), 3 );
//     std::vector<uint32_t> output( bufHost.size(), 10 );
       
//     memcpy( input.data(), bufDevice.data(), input.size() );
//     memcpy( output.data(), bufHost.data(), bufHost.size() );
//     std::cout << input[2] << std::endl;
//     std::cout << output[2] << std::endl;
// }