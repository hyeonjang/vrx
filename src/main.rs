#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vkcholesky/src/vkcholesky.h");

        fn initVulkan();
    }
}

use cxx;
use std::env;
use std::path::PathBuf;

fn main() {

    ffi::initVulkan();
}
