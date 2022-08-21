extern crate ash;
extern crate winit;

use ash::extensions::{
    ext::DebugUtils,
    khr::{Surface, Swapchain},
};
use ash::{vk, Entry};
use ash::{Device, Instance};
use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::c_char;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Vkdevice {
    pub entry: Entry,
    pub device: Device,
    pub instance: Instance,
    pub event_loop: RefCell<EventLoop<()>>,
}

impl Vkdevice {
    pub fn new() {
        unsafe {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title("Ash - Example")
                .with_inner_size(winit::dpi::LogicalSize::new(
                    f64::from(1.0f64),
                    f64::from(1.0f64),
                ))
                .build(&event_loop)
                .unwrap();
            let entry = Entry::load().unwrap();
            let app_name = CStr::from_bytes_with_nul_unchecked(b"vk-cholesky\0");
            let layer_names = [CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )];
            let layers_names_raw: Vec<*const c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let mut extension_names = ash_window::enumerate_required_extensions(&window)
                .unwrap()
                .to_vec();
            extension_names.push(DebugUtils::name().as_ptr());

            let appinfo = vk::ApplicationInfo::default();
            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                vk::InstanceCreateFlags::default()
            };

            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&appinfo)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names);

            let instance = entry
                .create_instance(&create_info, None)
                .expect("Instance create error");

            let surface = ash_window::create_surface(&entry, &instance, &window, None).unwrap();
            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("Physical Device error");
            let surface_loader = Surface::new(&entry, &instance);
            let (pdevice, queue_family_index) = physical_devices
                .iter()
                .find_map(|pdevice| {
                    instance
                        .get_physical_device_queue_family_properties(*pdevice)
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            let supports_graphic_and_surface =
                                info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                    && surface_loader
                                        .get_physical_device_surface_support(
                                            *pdevice,
                                            index as u32,
                                            surface,
                                        )
                                        .unwrap();
                            if supports_graphic_and_surface {
                                Some((*pdevice, index))
                            } else {
                                None
                            }
                        })
                })
                .expect("Couldn't find suitable device.");
        }
    }
}
