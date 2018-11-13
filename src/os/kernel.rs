/// kernel.rs is a wrapper around c_kernel.
use super::c_kernel as kernel;
use super::raw_c_types as c_type;

/// register a new block devive with the given name. The kernel
/// can decide a device number on its own.
pub fn register_blockdevice(device_name: &str) -> Result<i32, i32> {
    let str_ptr = device_name.as_ptr() as *const c_type::c_char;
    let dev_nr: i32;
    unsafe {
        // Don't really care about a device number, let the kernel
        // figure it out for us.
        dev_nr = kernel::register_blkdev(0, str_ptr) as i32;
    }
    if dev_nr <= 0 {
        // TODO: impelment proper error
        return Err(dev_nr);
    }
    Ok(dev_nr)
}

/// unregister the block device with given name and major number
pub fn unregister_blockdevice(dev_major: i32, dev_name: &str) -> Result<(), &str> {
    // Can only unregister positive major_dev numbers
    if dev_major <= 0 {
        return Err("Can't unregister negative block device number");
    }
    let str_ptr = dev_name.as_ptr() as *const c_type::c_char;
    unsafe {
        // Interestingly, even though register blkdev returns a c_int,
        // this expects a c_uint.
        kernel::unregister_blkdev(dev_major as c_type::c_uint, str_ptr);
    }
    Ok(())
}
