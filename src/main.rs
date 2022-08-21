mod vkdevice;
use vkdevice::*;

fn main() {
    unsafe {
        Vkdevice::new();
    };

    println!("Hello, world!");
}
