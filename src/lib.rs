//! A block device written in rust
#![feature(lang_items)]
#![deny(missing_docs, warnings)]
// Ignore warnings in bindings
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Obviously we can't use std in the kernel
#![no_std]
// Some of the generated bindings can be empty structs
// depending on kernel config, rust doesn't really like that
#![allow(improper_ctypes)]
// Setup the allocator wrapper for the kernel
#![feature(allocator_api)]

// bring os module in scope
#[macro_use]
mod os;
pub mod rust_behaviour;

use os::kernel;

#[global_allocator]
static ALLOCATOR: os::allocator::Allocator = os::allocator::Allocator {};

// const DEVICE_SIZE: i32 = 1024; // 1024 sectors

static mut DEVICE_MAJOR: i32 = 0;

static DEVICE_NAME: &'static str = "ramdev";

#[no_mangle]
/// Module entry point
pub fn init_module() -> i32 {
    println!("init");
    unsafe {
        DEVICE_MAJOR = kernel::register_blockdevice(DEVICE_NAME).unwrap();
    }
    return 0;
}

#[no_mangle]
/// Module exit point
pub fn cleanup_module() {
    println!("Clean up");
    cleanup();
    println!("exit");
}

fn cleanup() {
    unsafe {
        if !(DEVICE_MAJOR < 0) {
            kernel::unregister_blockdevice(DEVICE_MAJOR, DEVICE_NAME).unwrap();
        }
    }
}
