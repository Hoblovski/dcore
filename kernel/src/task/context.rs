use crate::trap::trap_return;

/// Kernel task context. Used in `__switch`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskContext {
    /// return address ( e.g. __restore ) of __switch ASM function
    ra: usize,
    /// kernel stack pointer of app
    sp: usize,
    /// callee saved registers:  s 0..11
    s: [usize; 12],
}

impl TaskContext {
    /// This does not represent any task. It should be used as a placeholder.
    pub fn create_blank() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    /// Create a task context that will return to `trap_return --> __restore --> sepc`.
    /// The sepc is specified in the TrapContext SSA of the task.
    ///
    /// sp: initial kernel sp used when __switch enters the task.
    pub fn create_trap_return(sp: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: trap_return as usize,
            sp: sp,
            s: [0; 12],
        }
    }
}
