use syscall::*;
use syscall::platform::nr::*;

pub fn get_pid() -> usize {
    let pid = unsafe { syscall0(GETPID) };
    pid
}
