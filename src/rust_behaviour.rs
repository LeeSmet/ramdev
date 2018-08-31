//! Fillers for missing language functions because we don't use std
use core;
use core::panic::PanicInfo;

#[no_mangle]
#[lang = "eh_personality"]
/// Function normally provided by the standard library
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
#[lang = "eh_unwind_resume"]
/// This may or may not be needed
pub extern "C" fn rust_eh_unwind_resume() {}

#[no_mangle]
#[panic_handler]
/// Define what a panic is
pub extern "C" fn rust_begin_panic(_: &PanicInfo) -> ! {
    // panic by doing a segfault
    // apparently this is not unsafe
    let _ = *(core::ptr::null::<i32>());

    // panicking like this tricks the kernel into thinking there is a bug,
    // so it should print us a nice stack trace :)

    // since the signature said that we never return...
    // we're here forever now :)
    loop {}
}
