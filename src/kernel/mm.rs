use core::alloc::Layout;

use buddy_system_allocator::LockedHeap;

use crate::{misc::rust_lang::panic, println};

const KERNEL_HEAP_SIZE : usize = 1024000; // 1MB

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap::<32> = LockedHeap::<32>::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn mm_init() {
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
    println!("mm_init complete");
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: Layout) -> ! {
    panic!("Heap allocation failed, layout = {:?}", layout);
}