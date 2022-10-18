use core::arch::asm;

/// Trap handling.
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    stval, stvec,
};

mod context;
use crate::config::*;
use crate::{syscall::syscall, task::TASK_MANAGER};
pub use context::TrapContext;

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
/// `trap_return --> __restore --> sepc (e.g. previous user instruction)`
/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
pub fn trap_return() -> ! {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
    let user_satp = TASK_MANAGER.current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let __restore = TRAMPOLINE + (__restore as usize - __alltraps as usize);
    unsafe {
        asm!(
            "fence.i",
            "jr {__restore}",               // jump to new addr of __restore asm function
            __restore = in(reg) __restore,
            in("a0") TRAP_CONTEXT,          // a0 = virt addr of Trap Context
            in("a1") user_satp,             // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    let ctx = TASK_MANAGER.current_trap_context();
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.sepc += 4;
            use context::regs_index as RI;
            ctx.gpr[RI::A0] = syscall(
                ctx.gpr[RI::A7],
                [ctx.gpr[RI::A0], ctx.gpr[RI::A1], ctx.gpr[RI::A2]],
            ) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            unimplemented!()
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            unimplemented!()
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            unimplemented!()
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return()
}
