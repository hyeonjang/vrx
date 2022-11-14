#include "vkcholesky.hpp"

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