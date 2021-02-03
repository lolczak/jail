#![feature(asm)]
#![feature(untagged_unions)]

mod linux;

use linux::syscall;
use linux::ptrace;
use crate::linux::syscall::*;
use std::process::exit;
use crate::linux::ptrace::*;
use crate::linux::types::IoVec;
use std::mem;

fn main() {
    let flags = CLONE_FILES | CLONE_FS | CLONE_IO;
    // let pid = syscall::clone(flags);
    let pid = syscall::fork();

    if (pid == 0) {
        // sys_ptrace(PTRACE_TRACEME, 0, 0, 0);
        runner();
    } else {
        sys_ptrace(PTRACE_ATTACH, pid as i32, 0, 0);
        // sys_ptrace(PTRACE_SYSCALL, pid as i32, 0, 0);
        println!("pid: {}", pid);
        println!("Hello, world! pid: {}, tid: {}", syscall::getpid(), syscall::gettid());
        let mut status: i64 = 0;
        loop {
            let result = syscall::waitpid(pid as i64, &mut status, 0);
            println!("Waiting result: {}, status: {}", result, status);
            if WIFEXITED(status)  {
                println!("exiting");
                break;
            }
            if status == 1407 {
                let mut registers: PtraceRegisters = get_registers(pid as i32);
                println!("{} Regs: {}", status, registers);
                println!("Syscall info: {}", get_syscall_info(pid as i32));
            }
            sys_ptrace(PTRACE_SYSCALL, pid as i32, 0, 0);
        }
    }
}

fn runner() {
    // println!("new thread");
    // let result = syscall::execve("/usr/bin/ls", vec!["ls", "-la", "--color=auto"], vec!["LS_COLORS=rs=0:di=38;5;33:ln=38;5;51:mh=00:pi=40;38;5;11:"]);
    // println!("result {}", result);
    syscall::getpid();
    //println!("Thread! pid: {}, tid: {}", syscall::getpid(), syscall::gettid());
}
