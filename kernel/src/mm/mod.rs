//! Memory management:
//! * rust heap allocation for kernel
//! * frame allocation
//! * address mapping

mod address;
mod frame;
mod heap;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame::{frame_alloc, init_frame, FrameTracker};
pub use heap::init_heap;
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};

pub fn init() {
    init_heap();
    init_frame();
    KERNEL_SPACE.lock().activate();
    println!("activated kernel space");
}
