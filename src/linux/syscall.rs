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

pub fn fork() -> usize {
    let pid = unsafe { syscall0(FORK) };
    pid
}

pub const CLONE_VM: usize = 0x00000100;
pub const CLONE_FS: usize = 0x00000200;
pub const CLONE_FILES: usize = 0x00000400;
pub const CLONE_SIGHAND: usize = 0x00000800;
pub const CLONE_PIDFD: usize = 0x00001000;
pub const CLONE_PTRACE: usize = 0x00002000;
pub const CLONE_VFORK: usize = 0x00004000;
pub const CLONE_PARENT: usize = 0x00008000;
pub const CLONE_THREAD: usize = 0x00010000;
pub const CLONE_NEWNS: usize = 0x00020000;
pub const CLONE_SYSVSEM: usize = 0x00040000;
pub const CLONE_SETTLS: usize = 0x00080000;
pub const CLONE_PARENT_SETTID: usize = 0x00100000;
pub const CLONE_CHILD_CLEARTID: usize = 0x00200000;
pub const CLONE_DETACHED: usize = 0x00400000;
pub const CLONE_UNTRACED: usize = 0x00800000;
pub const CLONE_CHILD_SETTID: usize = 0x01000000;
pub const CLONE_NEWCGROUP: usize = 0x02000000;
pub const CLONE_NEWUTS: usize = 0x04000000;
pub const CLONE_NEWIPC: usize = 0x08000000;
pub const CLONE_NEWUSER: usize = 0x10000000;
pub const CLONE_NEWPID: usize = 0x20000000;
pub const CLONE_NEWNET: usize = 0x40000000;
pub const CLONE_IO: usize = 0x80000000;

//   long clone(unsigned long flags, void *child_stack,
//                       int *ptid, int *ctid,
//                       unsigned long newtls);
pub fn clone(flags: usize) -> i64 {
    let result = unsafe { syscall5(56, flags, 0, 0, 0, 0) };
    result as i64
}