#![feature(asm)]

pub const GETPID: usize = 39;

fn main() {
    let pid = unsafe {
         syscall0(GETPID)
    };
    println!("Hello, world! {}", pid);

}

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}