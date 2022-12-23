// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

// use std::ptr;
// use std::path;
// use std::process::Command;

// // include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// // pub struct Shader {
// //     shader_path:&'static str,
// //     spv_path:&'static str,
// // }

// // impl Shader {
// //     fn new(str_path:&'static str) -> Shader {

// //         let p = Path::new(str_path);
// //         let spv = p.with_extension("spv");


// //         let bytecode = include_bytes!(str_path);
// //         let bytecode = Vec::<u8>::from(&bytecode[..]);
    
// //         unsafe {
// //         let (prefix, code, suffix) = bytecode.align_to::<u32>();
// //         if !prefix.is_empty() || !suffix.is_empty() {
// //             // return Err(anyhow!("None"));
// //         }
    
// //         // here bug give address to module
// //         let mut module_ = VkShaderModule_T { _unused:[0;0] };
// //         let mut modddd:VkShaderModule = &mut module_;
// //         let module:*mut VkShaderModule = &mut modddd;
    
// //         let info = VkShaderModuleCreateInfo {
// //             sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
// //             pNext: ptr::null(),
// //             flags: 0,
// //             codeSize: bytecode.len(),
// //             pCode:code.as_ptr(),
// //         };
// //             vkCreateShaderModule(g_device, &info, ptr::null(), module);
// //             // return module;
// //         }

// //         Shader {
// //             shader_path:str_path,
// //             spv_path:spv.to_str().unwrap()
// //         }
// //     }

// //     fn compile(self) -> Self {
// //         let glslc = Command::new("$VULKAN_SDK?bin/glslc")
// //                 .arg(self.shader_path)
// //                 .arg("-o")
// //                 .arg(self.spv_path)
// //                 .output()
// //                 .expect("failed to execute process");
// //         self
// //     }

// //     fn create_module(self) -> *mut VkShaderModule {
// //         let bytecode = include_bytes!(self.spv_path);
// //         let bytecode = Vec::<u8>::from(&bytecode[..]);
    
// //         unsafe {
// //         let (prefix, code, suffix) = bytecode.align_to::<u32>();
// //         if !prefix.is_empty() || !suffix.is_empty() {
// //             // return Err(anyhow!("None"));
// //         }
    
// //         // here bug give address to module
// //         let mut module_ = VkShaderModule_T { _unused:[0;0] };
// //         let mut modddd:VkShaderModule = &mut module_;
// //         let module:*mut VkShaderModule = &mut modddd;
    
// //         let info = VkShaderModuleCreateInfo {
// //             sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
// //             pNext: ptr::null(),
// //             flags: 0,
// //             codeSize: bytecode.len(),
// //             pCode:code.as_ptr(),
// //         };
// //             vkCreateShaderModule(g_device, &info, ptr::null(), module);
// //             return module;
// //         }
// //     }
// // }

// // this should be macro
// pub fn create_shader_module(bytecode:&'static [u8]) -> *mut VkShaderModule {
//     let bytecode = Vec::<u8>::from(&bytecode[..]);

//     unsafe {
//     let (prefix, code, suffix) = bytecode.align_to::<u32>();
//     if !prefix.is_empty() || !suffix.is_empty() {
//         // return Err(anyhow!("None"));
//     }

//     // here bug give address to module
//     let mut module_ = VkShaderModule_T { _unused:[0;0] };
//     let mut modddd:VkShaderModule = &mut module_;
//     let module:*mut VkShaderModule = &mut modddd;

//     let info = VkShaderModuleCreateInfo {
//         sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
//         pNext: ptr::null(),
//         flags: 0,
//         codeSize: bytecode.len(),
//         pCode:code.as_ptr(),
//     };
//         vkCreateShaderModule(g_device, &info, ptr::null(), module);
//         return module;
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn init_vulkan() {
//         unsafe {
//             initVulkan();
//         }
//     }

//     #[test]
//     fn t_buffer() {
//         unsafe {
//             initVulkan();
//         }
//         let bufferCreateInfo = VkBufferCreateInfo {
//             sType:VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
//             sharingMode: VK_SHARING_MODE_EXCLUSIVE,
//             size:1,
//             flags:0,
//             usage:VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32,
//             pQueueFamilyIndices: std::ptr::null(),
//             queueFamilyIndexCount: 0,
//             pNext: std::ptr::null(),
//         };

//         let buf = Buffer::new(bufferCreateInfo, 1);
//     }

//     #[test]
//     fn t_create_shader_module() {
//         unsafe {
//             initVulkan();
//             let bytecode = include_bytes!("./shader/cholesky.spv");
//             create_shader_module(bytecode);
//     }
//     }

//     #[test]
//     fn t_create_descriptor() {
//         unsafe {
//             initVulkan();
//             let _desc = Descriptor::new(1);
//         }
//     }

//     #[test]
//     fn t_create_pipeline() {
//         unsafe {
//             initVulkan();
//             let desc = Descriptor::new(1);
//             let mut pipeline = ComputePipeline::new();
//             pipeline.createPipelineLayout(desc);
//             // pipeline.createPipeline();

            
//         }
//     }

// }
