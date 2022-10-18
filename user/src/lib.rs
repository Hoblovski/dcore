#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use syscall::{sys_exit, sys_write};

#[macro_use]
pub mod uconsole;
mod lang_items;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    // bss pages are zeroed upon allocation by FrameTracker
    exit(main());
}

#[allow(dead_code)]
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code)
}

fn write(fd: usize, data: &[u8]) -> isize {
    sys_write(fd, data)
}
