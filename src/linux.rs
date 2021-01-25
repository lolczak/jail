use syscall::*;
use syscall::platform::nr::*;

pub fn getpid() -> usize {
    let pid = unsafe { syscall0(GETPID) };
    pid
}

pub fn gettid() -> usize {
    let pid = unsafe { syscall0(GETTID) };
    pid
}
