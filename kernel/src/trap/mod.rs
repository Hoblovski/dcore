/// Trap handling.
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    stval, stvec,
};

mod context;
use crate::syscall::syscall;
pub use context::TrapContext;

use core::arch::global_asm;
global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            use context::regs_index as RI;
            cx.gpr[RI::A0] = syscall(
                cx.gpr[RI::A7],
                [cx.gpr[RI::A0], cx.gpr[RI::A1], cx.gpr[RI::A2]],
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
    cx
}
