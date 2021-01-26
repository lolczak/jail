#![feature(asm)]

mod linux;

use linux::syscall;

fn main() {
    let pid = syscall::fork();
    println!("pid: {}", pid);
    println!("Hello, world! pid: {}, tid: {}", syscall::getpid(), syscall::gettid());
}
