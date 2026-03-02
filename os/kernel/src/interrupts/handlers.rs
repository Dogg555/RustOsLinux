use core::arch::asm;

use crate::drivers::{keyboard, pic};
use crate::timer;

#[no_mangle]
pub extern "x86-interrupt" fn timer_interrupt() {
    timer::tick();
    pic::end_of_interrupt(0);
}

#[no_mangle]
pub extern "x86-interrupt" fn keyboard_interrupt() {
    keyboard::handle_interrupt();
    pic::end_of_interrupt(1);
}

#[no_mangle]
pub extern "x86-interrupt" fn page_fault_handler() {
    let fault_addr: u32;
    unsafe {
        asm!("mov {0:e}, cr2", out(reg) fault_addr, options(nostack, preserves_flags));
    }
    println!("Page fault at: {:#x}", fault_addr);
    loop {
        x86::halt();
    }
}

#[no_mangle]
pub extern "x86-interrupt" fn double_fault_handler() {
    println!("Double fault");
    loop {
        x86::halt();
    }
}
