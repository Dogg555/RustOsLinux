#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod arch;
mod drivers;
mod interrupts;
mod memory;
mod timer;
mod vga;

use core::panic::PanicInfo;
use x86::irq;

core::arch::global_asm!(include_str!("start.S"));

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_magic: u32, multiboot_info: u32) -> ! {
    vga::init();
    println!("RustOS Kernel Booted");

    if multiboot_magic != 0x36d76289 {
        println!("Invalid multiboot magic: {:#x}", multiboot_magic);
        loop {}
    }

    arch::x86::gdt::init();
    interrupts::idt::init();
    drivers::pic::init();
    timer::init(100);
    memory::init(multiboot_info as usize);

    unsafe { irq::enable() };

    println!("Timer running");
    println!("Interrupts enabled");

    let mut last_tick = 0;
    loop {
        let ticks = timer::uptime_ticks();
        if ticks != last_tick && ticks % 100 == 0 {
            println!("uptime ticks: {}", ticks);
            last_tick = ticks;
        }
        x86::halt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nKERNEL PANIC: {}", info);
    loop {
        x86::halt();
    }
}
