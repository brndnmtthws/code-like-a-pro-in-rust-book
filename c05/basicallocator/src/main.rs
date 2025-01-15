#![feature(allocator_api)]

use std::alloc::{AllocError, Allocator, Layout};
use std::ptr::NonNull;

use libc::{free, malloc};

pub struct BasicAllocator;

unsafe impl Allocator for BasicAllocator {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let ptr = malloc(layout.size() as libc::size_t);
            let slice = std::slice::from_raw_parts_mut(
                ptr as *mut u8,
                layout.size(),
            );
            Ok(NonNull::new_unchecked(slice))
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        free(ptr.as_ptr() as *mut libc::c_void);
    }
}

fn main() {
    let mut custom_alloc_vec: Vec<i32, _> =
        Vec::with_capacity_in(10, BasicAllocator);
    for i in 0..10 {
        custom_alloc_vec.push(i + 1);
    }
    println!("custom_alloc_vec={:?}", custom_alloc_vec);
}
