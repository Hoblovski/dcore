#![macro_use]
use alloc::{vec, vec::Vec};
use lazy_static::*;
use spin::Mutex;

mod context;
use crate::{
    config::*,
    mm::{MapPermission, MemorySet, PhysPageNum, VirtAddr, KERNEL_SPACE},
    trap::{trap_handler, TrapContext},
};
use context::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
}

pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub ctx: TaskContext,
    /// User level memory set
    memory_set: MemorySet,
    /// TrapContext state save area. Page-aligned.
    trap_ctx_ppn: PhysPageNum,
}

// TODO: task id is simply its index within the tasks Vec.
pub struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

pub struct TaskManager {
    inner: Mutex<TaskManagerInner>,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let status = TaskStatus::Ready;
        // map a kernel-stack in kernel space

        let kernel_stack_high = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
        let kernel_stack_low = kernel_stack_high - KERNEL_STACK_SIZE;
        KERNEL_SPACE.lock().insert_framed_area(
            kernel_stack_low.into(),
            kernel_stack_high.into(),
            MapPermission::R | MapPermission::W,
        );
        let task_control_block = Self {
            status,
            ctx: TaskContext::create_trap_return(kernel_stack_high),
            memory_set,
            trap_ctx_ppn,
        };
        // prepare TrapContext in user space
        let trap_ctx = task_control_block.get_trap_context();
        *trap_ctx = TrapContext::create_user(
            entry_point,
            user_sp,
            KERNEL_SPACE.lock().token(),
            kernel_stack_high,
            trap_handler as usize,
        );
        task_control_block
    }

    fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }

    fn get_trap_context(&self) -> &'static mut TrapContext {
        self.trap_ctx_ppn.get_mut()
    }
}

impl TaskManager {
    pub fn run_first(&self) -> ! {
        extern "C" {
            /// Save task state to *current, and switch to task specified by *next.
            pub fn __switch(current_ctx: *mut TaskContext, next_ctx: *const TaskContext);
        }
        let mut inner = self.inner.lock();
        let next_task = &mut inner.tasks[0];
        next_task.status = TaskStatus::Running;
        let next_task_ctx_ptr = &next_task.ctx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::create_blank();
        unsafe {
            __switch(&mut _unused as *mut _, next_task_ctx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    pub fn finish_one(&self) -> ! {
        println!("Done for now...");
        loop {}
    }

    pub fn current_user_token(&self) -> usize {
        let inner = self.inner.lock();
        inner.tasks[inner.current_task].get_user_token()
    }

    pub fn current_trap_context(&self) -> &'static mut TrapContext {
        let inner = self.inner.lock();
        inner.tasks[inner.current_task].get_trap_context()
    }
}

lazy_static! {
    /// Global variable: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let tasks = vec![
        TaskControlBlock::new(get_app_data(0), 0)
        ];
        let inner = TaskManagerInner {
            tasks: tasks,
            current_task: 0
        };
        TaskManager {
            inner: Mutex::new(inner)
        }
    };
}

// Retrieves app info from linked elf binary.
// Serves like a file system.
// TODO: put in file system subsystem
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
