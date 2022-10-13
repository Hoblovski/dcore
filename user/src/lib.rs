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
    clear_bss();
    exit(main());
}

#[allow(dead_code)]
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code)
}

fn write(fd: usize, data: &[u8]) -> isize {
    sys_write(fd, data)
}