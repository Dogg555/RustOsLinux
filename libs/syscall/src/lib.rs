#![no_std]

#[repr(u16)]
pub enum Syscall {
    Yield = 0,
    Send = 1,
    Receive = 2,
    Spawn = 3,
}
