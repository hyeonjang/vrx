mod vkdevice;
use vkdevice::*;

fn main() {
    unsafe {
        VkContext::new();
    };

    println!("Hello, world!");
}
