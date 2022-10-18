use crate::config::*;
/// The heap used by the Rust language i.e. `alloc` routines.
use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    // TODO: maybe frame-alloc and supply with new space?
    panic!("Heap allocation error, layout = {:?}", layout);
}

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_mut_ptr(), KERNEL_HEAP_SIZE);
    }
    println!("heap: {} bytes", KERNEL_HEAP_SIZE);
}
