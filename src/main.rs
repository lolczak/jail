#![feature(asm)]

mod linux;

use linux::syscall;
use crate::linux::syscall::{CLONE_THREAD, CLONE_FILES, CLONE_FS, CLONE_IO, CLONE_PIDFD, CLONE_NEWPID};
use std::process::exit;

fn main() {
    let flags = CLONE_FILES | CLONE_FS | CLONE_IO;
    let pid = syscall::clone(flags);

    if (pid == 0) {
        runner();
    } else {
        println!("pid: {}", pid);
        println!("Hello, world! pid: {}, tid: {}", syscall::getpid(), syscall::gettid());
    }
}

fn runner() {
    println!("new thread")
}
