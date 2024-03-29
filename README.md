﻿# vrx
### Low-level
#### No wrapping for vulkan-primitive by rust

Rust has the special feature "trait" that is the compile-time function table.
In vrx, we can just call OOP method without any wrapping struct and class, supported by this.
```rust
// 1. original method to call vulkan functions
let device: VkDevice = /* some initialization ...*/;

let mut buffer: VkBuffer;
let buffer_create_info = VkBufferCreateInfo { ... };
vkCreateBuffer(device, &buffer_create_info, std::ptr::null(), &buffer);
```
```rust 
// 2. new trait-based method to call vulkan functions
let buffer = device.create_buffer(&buffer_create_info, None);
```
In addition, we support the Domain Specific Launguage (DSL) for VkCommandBuffer, which also aims not to harm the communication with vulkan-primitive types [reference post](https://blog-an.vercel.app/DSL-vkCommand).

```rust 
let command: VkCommandBuffer = /*omit detials*/; // created by vulkan devices
vkCmdBlock! {
	THIS command

	let copy_buffer = VkBufferCopy { srcOffset: 0, dstOffset: 0, size: buffer_size };
	COPY_BUFFER(src_buffer, dst_buffer, 0, &copy_buffer);

	/* some other vulkan commands */
	/*            ...             */
}
```

### High-level
#### Simple method to build GPU pipeline

## Roadmap

## install
Initial vulkan binding
supported by [bindgen](https://rust-lang.github.io/rust-bindgen/)

bindgen requirements
https://rust-lang.github.io/rust-bindgen/requirements.html

vulkan sdk downloads
https://vulkan.lunarg.com/sdk/home#windows

## task
Minimal binding for vulkan
