/// Physical frame allocation.
use super::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use lazy_static::*;
use spin::Mutex;

/// The tracker should have the same lifecycle as the frame.
/// Drop only when the frame is to be released.
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    /// Zeroing the allocated frame on initialization.
    pub fn new(ppn: PhysPageNum) -> Self {
        ppn.get_bytes_array().fill(0);
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

/// Naive allocator.
pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, begin_ppn: PhysPageNum, end_ppn: PhysPageNum) {
        self.current = begin_ppn.0;
        self.end = end_ppn.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current == self.end {
            None
        } else {
            self.current += 1;
            Some((self.current - 1).into())
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        self.recycled.push(ppn);
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    static ref FRAME_ALLOCATOR: Mutex<FrameAllocatorImpl> = Mutex::new(FrameAllocatorImpl::new());
}

pub fn init_frame() {
    extern "C" {
        fn ekernel();
    }
    let begin_ppn = PhysAddr::from(ekernel as usize).ceil();
    let end_ppn = PhysAddr::from(MEMORY_END).floor();
    FRAME_ALLOCATOR.lock().init(begin_ppn, end_ppn);
    println!(
        "frames: PPN {:#x} ~ {:#x}. {} frames",
        begin_ppn.0,
        end_ppn.0,
        end_ppn.0 - begin_ppn.0
    );
}

/// allocate a frame
pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.lock().alloc().map(FrameTracker::new)
}

/// deallocate a frame
fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.lock().dealloc(ppn);
}
