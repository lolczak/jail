#![feature(asm)]

mod linux;

use linux::syscall;
use crate::linux::syscall::{CLONE_THREAD, CLONE_FILES, CLONE_FS, CLONE_IO, CLONE_PIDFD, CLONE_NEWPID, WSTOPPED, WEXITED, WUNTRACED, WCONTINUED};
use std::process::exit;

fn main() {
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
    println!("new thread")
}
