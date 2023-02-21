extern crate winit;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::LogicalSize, platform::windows::WindowExtWindows,
};
use vrx::*;

fn main() {
    println!("Hello, world!");    
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vk test")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)
        .unwrap();
    
    let win32_surface_create_info = VkWin32SurfaceCreateInfoKHRBuilder::new()
        .hinstance(window.hinstance() as vrx::HINSTANCE)
        .hwnd(window.hwnd() as vrx::HWND)
        .build();
}
