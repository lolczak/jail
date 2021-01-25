#![feature(asm)]

mod linux;

fn main() {

    println!("Hello, world! pid: {}, tid: {}", linux::getpid(), linux::gettid());

}
