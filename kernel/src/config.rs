/// Configuration values.
#[allow(unused)]
pub use crate::consts::*;

// Loader
pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;

// Memory
pub const KERNEL_HEAP_SIZE: usize = 1024 * 1024;
/// The maximum memory address available.
pub const MEMORY_END: usize = 0x80800000;
/// The virtual page address at which trampoline code is mapped.
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
/// The virtual page address at which TrapContext state save area resides.
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
