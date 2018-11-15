// TODO: Fix so we can import from generated bindings
extern "C" {
    pub fn printk(fmt: *const u8, ...) -> i32;
}

/// Printing macro. This adds a zero character to the string to terminate it
macro_rules! print {
    ($e:expr) => {
        unsafe {
            os::print::printk(concat!($e, "\0").as_ptr());
        }
    };
}

/// Printing macro which appens a newline
macro_rules! println {
    ($e:expr) => {
        print!(concat!($e, "\n"))
    };
}
