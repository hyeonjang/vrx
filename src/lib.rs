#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn create_shader_module(spv_path:&'static str) -> *mut VkShaderModule {
    let bytecode = include_bytes!("./shader/cholesky.spv");
    let bytecode = Vec::<u8>::from(&bytecode[..]);

    unsafe {
    let (prefix, code, suffix) = bytecode.align_to::<u32>();
    if !prefix.is_empty() || !suffix.is_empty() {
        // return Err(anyhow!("None"));
    }

    // here bug give address to module
    let module = std::ptr::null_mut();
    let info = VkShaderModuleCreateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        codeSize: bytecode.len(),
        pCode:code.as_ptr(),
    };

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
    fn t_create_shader_module() {
        unsafe {
            initVulkan();
            create_shader_module(" ");
    }
    }
}
