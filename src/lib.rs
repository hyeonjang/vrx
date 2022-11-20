#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn create_shader_module() -> *mut VkShaderModule {

    unsafe {
    let module = VkShaderModule {};
    let info = VkShaderModuleCreateInfo { sType:VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO, pNext:ptr::null(), flags:0, codeSize:0, pCode:ptr::null() };

        vkCreateShaderModule(g_device, &info, ptr::null(), module);
        return module;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_vulkan() {
        unsafe {
            initVulkan();
        }
    }

    #[test]
    fn create_shader_module() {
        unsafe {
            let module:VkShaderModule;
            vkCreateShaderModule
        }
    }

}