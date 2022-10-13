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
    pub fn create_empty() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    /// Create a task context that will return to `__restore`,
    /// and then returning to the SEPC in the trap context there.
    ///
    /// * `sp`: a `TrapContext` must reside at address `sp`
    pub fn create_restore(sp: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: sp,
            s: [0; 12],
        }
    }
}
