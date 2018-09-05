//! A block device written in rust
#![feature(lang_items)]
#![feature(panic_implementation)]
#![deny(missing_docs, warnings)]
#![feature(panic_handler)]
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

// bring os module in scope
mod os;

#[macro_use]
mod print;
pub mod rust_behaviour;

use os::kernel;
use os::raw_c_types as c_type;

// const DEVICE_SIZE: i32 = 1024; // 1024 sectors

static mut DEVICE_MAJOR: c_type::c_int = 0;

#[no_mangle]
/// Module entry point
pub fn init_module() -> c_type::c_int {
    println!("init");
    unsafe {
        DEVICE_MAJOR = kernel::register_blkdev(0, b"test" as *const c_type::c_char);
    }
    return 0;
}

#[no_mangle]
/// Module exit point
pub fn cleanup_module() {
    println!("Clean up");
    // The if check is also unsafe as it accesses a mutable global variable
    unsafe {
        if !(DEVICE_MAJOR < 0) {
            kernel::unregister_blkdev(
                DEVICE_MAJOR as c_type::c_uint,
                b"test" as *const c_type::c_char,
            );
        }
    }
    println!("exit");
}
