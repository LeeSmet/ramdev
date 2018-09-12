use super::c_kernel;
use super::raw_c_types as c_type;
use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Ideally we'd have a kmalloc binding in the kernel. However kmalloc
        // is marked as inline. Fortunately, krealloc, is explicitly defined
        // to have the same functionality as kmalloc, if it is given a null
        // pointer to re allocate
        c_kernel::krealloc(
            0 as *const c_type::c_void,
            layout.size(),
            // GFP_KERNEL is not generated since its a macro which should do the
            // following
            c_kernel::___GFP_RECLAIMABLE | c_kernel::___GFP_IO | c_kernel::___GFP_FS,
        ) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        c_kernel::kfree(ptr as *const c_type::c_void);
    }
}
