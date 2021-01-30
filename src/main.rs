#![feature(asm)]

mod linux;

use linux::syscall;
use crate::linux::syscall::{CLONE_THREAD, CLONE_FILES, CLONE_FS, CLONE_IO, CLONE_PIDFD, CLONE_NEWPID, WSTOPPED, WEXITED, WUNTRACED, WCONTINUED, execve};
use std::process::exit;

fn main() {
    let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();

    let flags = CLONE_FILES | CLONE_FS | CLONE_IO;
    // let pid = syscall::clone(flags);
    let pid = syscall::fork();

    if (pid == 0) {
        runner();
    } else {
        println!("pid: {}", pid);
        println!("Hello, world! pid: {}, tid: {}", syscall::getpid(), syscall::gettid());
        let mut status: i64 = 0;
        let result = syscall::waitpid(pid as i64, &mut status, WUNTRACED | WCONTINUED);
        println!("Waiting result: {}, status: {}", result, status);
    }
}

fn runner() {
    println!("new thread");
    let result = syscall::execve("/usr/bin/ls", vec!["ls", "-la", "--color=auto"], vec!["LS_COLORS=rs=0:di=38;5;33:ln=38;5;51:mh=00:pi=40;38;5;11:"]);
    println!("result {}", result);
}
