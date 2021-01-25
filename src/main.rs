#![feature(asm)]

mod linux;

fn main() {

    println!("Hello, world! {}", linux::get_pid());

}
