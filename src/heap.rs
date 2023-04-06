/*
Author: Ben Mezger (github.com/benmezger)
*/

use core::alloc::{GlobalAlloc, Layout};

struct GHeapAllocator;

#[cfg(test)]
static mut CALL_COUNT: usize = 0;

unsafe impl GlobalAlloc for GHeapAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                CALL_COUNT += 1;
            }
        }

        crate::arch::kmem::kzmalloc(_layout.size())
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static G: GHeapAllocator = GHeapAllocator {};

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!(
        "Allocation error of {} bytes with {}-byte alignment",
        layout.size(),
        layout.align(),
    )
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        extern crate alloc;
        use alloc::string;

        #[test_case]
        fn test_global_alloc() {
            crate::arch::mem::init();
            crate::arch::kmem::init();
            string::String::from("Should call alloc once.");

            unsafe {
                assert_eq!(CALL_COUNT, 1);
            }
        }
    }
}
