extern crate bindgen;

use std::path::Path;

fn main() {
    // println!("cargo:rustc-link-lib=bz2");

    let bindings = bindgen::Builder::default()
        .header("src/vkcontext.hpp")
        .clang_arg("-IC:/VulkanSDK/1.3.216.0/Include")
        .generate()
        .expect("Unable to generate bindings");

    // let out_path = let::from(env::var("C:/Users/hyeon/bindgen-tutorial-bzip2-sys").unwrap());

    let out_path = Path::new("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
