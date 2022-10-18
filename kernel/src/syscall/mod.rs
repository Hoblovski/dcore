/// Implementation of syscall dispatching.
use crate::{mm::translated_byte_buffer, task::TASK_MANAGER};

mod syscall_no {
    pub const WRITE: usize = 64;
    pub const EXIT: usize = 93;
    pub const YIELD: usize = 124;
    pub const GET_TIME: usize = 169;
}

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    println!("syscall {} got.", syscall_id);
    use syscall_no as NO;
    match syscall_id {
        NO::WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        NO::EXIT => sys_exit(args[0] as i32),
        NO::YIELD => unimplemented!(),
        NO::GET_TIME => unimplemented!(),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(TASK_MANAGER.current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd {} in sys_write!", fd);
        }
    }
}

fn sys_exit(_exit_code: i32) -> ! {
    TASK_MANAGER.finish_one()
}
