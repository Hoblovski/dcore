#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod config;
mod lang_items;
mod sbi;
mod syscall;
mod task;
mod trap;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("Early hello from kernel.");
    // init_heap();
    // println!("Heap inited. heap_address: {:?}", unsafe { &HEAP_SPACE.as_ptr_range() });

    trap::init();
    task::load_apps();
    println!("apps loaded");
    let f = &task::TASK_MANAGER;
    f.run_first();
}
