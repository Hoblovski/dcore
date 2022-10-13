use lazy_static::*;
use spin::Mutex;

mod context;
use crate::config::*;
use context::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
}

pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub ctx: TaskContext,
}

pub struct TaskManagerInner {
    task: TaskControlBlock,
}

pub struct TaskManager {
    inner: Mutex<TaskManagerInner>,
}

use core::arch::{asm, global_asm};
global_asm!(include_str!("switch.S"));

impl TaskManager {
    pub fn run_first(&self) -> ! {
        extern "C" {
            /// Save task state to *current, and switch to task specified by *next.
            pub fn __switch(current_ctx: *mut TaskContext, next_ctx: *const TaskContext);
        }
        let mut inner = self.inner.lock();
        let task = &mut inner.task;
        task.status = TaskStatus::Running;
        let next_task_cx_ptr = &task.ctx as *const TaskContext;
        drop(inner);

        let mut bootctx = TaskContext::create_empty();
        println!("switching");
        unsafe {
            __switch(&mut bootctx as *mut TaskContext, next_task_cx_ptr);
        }
        unreachable!()
    }

    pub fn finish_one(&self) -> ! {
        println!("Done for now...");
        loop {}
    }
}

lazy_static! {
    /// Global variable: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let inner = TaskManagerInner {
            task: TaskControlBlock {
                status: TaskStatus::Ready,
                ctx:TaskContext::create_restore(init_kern_stack(0)) }
        };
        TaskManager {
            inner: Mutex::new(inner)
        }
    };
}

// Temporary Loader
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// Copy the application binary into its load address.
pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }
    let num_app = get_num_app();
    assert_eq!(num_app, 1);
    // We're updating coherence-less imem so manually flush icache.
    unsafe {
        asm!("fence.i");
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    // load app
    (APP_BASE_ADDRESS..APP_BASE_ADDRESS + APP_SIZE_LIMIT)
        .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
    let src = unsafe {
        core::slice::from_raw_parts(app_start[0] as *const u8, app_start[1] - app_start[0])
    };
    assert!(app_start[1] - app_start[0] < APP_SIZE_LIMIT);
    let dst = unsafe { core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, src.len()) };
    dst.copy_from_slice(src);
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

/// Initialize contents of kernel stack for application app_id.
///   Simply puts a TrapContext to the stack.
/// * ret: `sp` that points to top of the stack
use crate::trap::TrapContext;
pub fn init_kern_stack(app_id: usize) -> usize {
    let kern_stk = &KERNEL_STACK[app_id];
    let top = kern_stk.data.as_ptr_range().end as usize;
    let top = (top - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
    let user_stk_top = USER_STACK[app_id].data.as_ptr_range().end as usize;
    let trap_ctx =
        TrapContext::create_init_user(APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT, user_stk_top);
    unsafe {
        *top = trap_ctx;
    }
    top as usize
}
