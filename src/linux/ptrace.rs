use syscall::*;
use syscall::platform::nr::*;

use crate::linux::types::pid_t;
use std::ffi::c_void;
use std::fmt;
use std::mem;
use std::fmt::Formatter;
use std::borrow::Borrow;
use crate::linux::types::IoVec;
//include/uapi/linux/ptrace.h
//arch/x86/include/uapi/asm/ptrace-abi.h
//arch/x86/include/uapi/asm/ptrace.h
//tools/testing/selftests/ptrace/peeksiginfo.c

pub const PTRACE_TRACEME: usize = 0;
pub const PTRACE_PEEKTEXT: usize = 1;
pub const PTRACE_PEEKDATA: usize = 2;
pub const PTRACE_PEEKUSR: usize = 3;
pub const PTRACE_POKETEXT: usize = 4;
pub const PTRACE_POKEDATA: usize = 5;
pub const PTRACE_POKEUSR: usize = 6;
pub const PTRACE_CONT: usize = 7;
pub const PTRACE_KILL: usize = 8;
pub const PTRACE_SINGLESTEP: usize = 9;
pub const PTRACE_ATTACH: usize = 16;
pub const PTRACE_DETACH: usize = 17;
pub const PTRACE_SYSCALL: usize = 24;

pub const PTRACE_SETOPTIONS: usize = 0x4200;
pub const PTRACE_GETEVENTMSG: usize = 0x4201;
pub const PTRACE_GETSIGINFO: usize = 0x4202;
pub const PTRACE_SETSIGINFO: usize = 0x4203;

pub const PTRACE_GETREGSET: usize = 0x4204;
pub const PTRACE_SETREGSET: usize = 0x4205;
pub const PTRACE_SEIZE: usize = 0x4206;
pub const PTRACE_INTERRUPT: usize = 0x4207;
pub const PTRACE_LISTEN: usize = 0x4208;
pub const PTRACE_PEEKSIGINFO: usize = 0x4209;

pub const PTRACE_GETSIGMASK: usize = 0x420a;
pub const PTRACE_SETSIGMASK: usize = 0x420b;
pub const PTRACE_SECCOMP_GET_FILTER: usize = 0x420c;
pub const PTRACE_SECCOMP_GET_METADATA: usize = 0x420d;

pub const PTRACE_GET_SYSCALL_INFO: usize = 0x420e;
pub const PTRACE_SYSCALL_INFO_NONE: usize = 0;
pub const PTRACE_SYSCALL_INFO_ENTRY: usize = 1;
pub const PTRACE_SYSCALL_INFO_EXIT: usize = 2;
pub const PTRACE_SYSCALL_INFO_SECCOMP: usize = 3;

pub const PTRACE_EVENTMSG_SYSCALL_ENTRY: usize = 1;
pub const PTRACE_EVENTMSG_SYSCALL_EXIT: usize = 2;

pub const PTRACE_PEEKSIGINFO_SHARED: usize = 1 << 0;

pub const PTRACE_EVENT_FORK: usize = 1;
pub const PTRACE_EVENT_VFORK: usize = 2;
pub const PTRACE_EVENT_CLONE: usize = 3;
pub const PTRACE_EVENT_EXEC: usize = 4;
pub const PTRACE_EVENT_VFORK_DONE: usize = 5;
pub const PTRACE_EVENT_EXIT: usize = 6;
pub const PTRACE_EVENT_SECCOMP: usize = 7;
pub const PTRACE_EVENT_STOP: usize = 128;

pub const SYS_PTRACE: usize = 101;

#[repr(C)]
#[derive(Debug, Default)]
pub struct PtraceRegisters {
    r15      : u64,
    r14      : u64,
    r13      : u64,
    r12      : u64,
    rbp      : u64,
    rbx      : u64,
    r11      : u64,
    r10      : u64,
    r9       : u64,
    r8       : u64,
    pub rax      : u64,
    rcx      : u64,
    rdx      : u64,
    rsi      : u64,
    rdi      : u64,
    pub orig_rax : u64,
    rip      : u64,
    cs       : u64,
    eflags   : u64,
    rsp      : u64,
    ss       : u64,
    fs_base  : u64,
    gs_base  : u64,
    ds       : u64,
    es       : u64,
    fs       : u64,
    gs       : u64
}

impl fmt::Display for PtraceRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "orig_rax: {}, rax: {}, rdi: {}, rsi: {}, rdx: {}, rip: {}", self.orig_rax, (self.rax as i64), self.rdi, self.rsi, self.rdx, self.rip)
    }
}

#[repr(C)]
pub struct PtraceSyscallInfo {
    op: u8,
    arch: u32,
    instruction_pointer: u64,
    stack_pointer: u64,
    info: SyscallInfo
}

#[repr(C)]
union SyscallInfo {
    entry: EntryInfo,
    exit: ExitInfo,
    seccomp: SeccompInfo
}

#[repr(C)]
#[derive(Debug, Default)]
struct EntryInfo {
    nr: u64,
    args: [u64; 6]
}

#[repr(C)]
#[derive(Debug, Default)]
struct ExitInfo {
    rval: i64,
    is_error: u8
}

#[repr(C)]
#[derive(Debug, Default)]
struct SeccompInfo {
    nr: u64,
    args: [u64; 6],
    ret_data: u32
}

impl fmt::Display for PtraceSyscallInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "op: {}, arch: {}, ip: {}, sp: {}", self.op, self.arch, self.instruction_pointer, self.stack_pointer);
        if (self.op == (PTRACE_SYSCALL_INFO_ENTRY as u8)) {
            let nr = unsafe { self.info.entry.nr };
            let args = unsafe { self.info.entry.args };
            write!(f, "nr: {}, args: {}",nr , args[0])
        } else {
            write!(f, " none")
        }
    }
}


// pub fn sys_ptrace(request: usize, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> i64 {
pub fn sys_ptrace(request: usize, pid: pid_t, addr: usize, data: usize) -> i64 {
    let result = unsafe { syscall4(SYS_PTRACE, request, pid as usize, addr, data) };
    result as i64
}

pub fn get_syscall_info(pid: pid_t) -> PtraceSyscallInfo {
    let mut syscall_info = PtraceSyscallInfo { op:1, arch:0, instruction_pointer:0,stack_pointer:0,info:SyscallInfo { seccomp: SeccompInfo::default() } };
    let address =  &syscall_info as *const _;
    let length = mem::size_of::<PtraceSyscallInfo>();
    let result = sys_ptrace(PTRACE_GET_SYSCALL_INFO, pid, length, address as usize);

    syscall_info
}

pub fn get_registers(pid: pid_t) -> PtraceRegisters {
    let mut registers: PtraceRegisters = PtraceRegisters::default();
    let address = &registers as *const _;
    let length = mem::size_of::<PtraceRegisters>();
    let mut iovec: IoVec = IoVec { base: address as usize, len: length as u64 };
    let data_ptr = &iovec as *const _;
    let res = sys_ptrace(PTRACE_GETREGSET, pid as i32, 1, data_ptr as usize);

    registers
}

pub fn set_registers(pid: pid_t, registers: PtraceRegisters) -> i64 {
    let address = &registers as *const _;
    let length = mem::size_of::<PtraceRegisters>();
    let mut iovec: IoVec = IoVec { base: address as usize, len: length as u64 };
    let data_ptr = &iovec as *const _;
    let res = sys_ptrace(PTRACE_SETREGSET, pid as i32, 1, data_ptr as usize);
    res
}