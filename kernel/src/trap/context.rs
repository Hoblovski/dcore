use riscv::register::sstatus::{self, Sstatus, SPP};

#[allow(unused)]
pub mod regs_index {
    pub const SP: usize = 2;

    pub const A0: usize = 10;
    pub const A1: usize = 11;
    pub const A2: usize = 12;
    pub const A3: usize = 13;
    pub const A4: usize = 14;
    pub const A5: usize = 15;
    pub const A6: usize = 16;
    pub const A7: usize = 17;
}

/// Trap context. Used in `__alltraps` and `__restore`.
#[repr(C)]
pub struct TrapContext {
    pub gpr: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    /// Create initial trap context for a user program.
    /// Control flow enters user program via `__restore`.
    ///
    /// * `entry`: entry address.
    /// * `sp`: points to the end of user stack.
    pub fn create_init_user(entry: usize, stack_top: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            gpr: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.gpr[regs_index::SP] = stack_top;
        cx
    }
}
