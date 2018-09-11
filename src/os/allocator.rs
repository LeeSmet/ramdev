use super::kernel;
use super::raw_c_types as c_type;
use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Ideally we'd have a kmalloc binding in the kernel. However kmalloc
        // is marked as inline. Fortunately, krealloc, is explicitly defined
        // to have the same functionality as kmalloc, if it is given a null
        // pointer to re allocate
        kernel::krealloc(
            0 as *const c_type::c_void,
            layout.size(),
            // ___GFP_KERNEL is not generated since its a macro which does the
            // following
            kernel::___GFP_RECLAIMABLE | kernel::___GFP_IO | kernel::___GFP_FS,
        ) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        kernel::kfree(ptr as *const c_type::c_void);
    }
}
