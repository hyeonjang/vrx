mod vkdevice;
use vkdevice::*;

fn main() {
    unsafe {
        let vkcontext = VkContext::new();
        let device = vkcontext.device();

        let compute_pipeline = VkCompute::new(&device);

    };


    println!("Hello, world!");
}
