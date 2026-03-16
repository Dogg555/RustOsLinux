#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod arch;
mod drivers;
mod filesystem;
mod graphics;
mod interrupts;
mod ipc;
mod logging;
mod memory;
mod networking;
mod process;
mod scheduler;
mod security;
mod syscalls;
mod timer;
mod vga;

use core::panic::PanicInfo;
use logging::LogLevel;
use x86::irq;

core::arch::global_asm!(include_str!("start.S"));

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_magic: u32, multiboot_info: u32) -> ! {
    vga::init();
    logging::init();
    klog!(LogLevel::Info, "RustOS Kernel Booted");

    if multiboot_magic != 0x36d76289 {
        klog!(
            LogLevel::Error,
            "Invalid multiboot magic: {:#x}",
            multiboot_magic
        );
        loop {}
    }

    arch::x86::gdt::init();
    interrupts::idt::init();
    drivers::pic::init();
    timer::init(100);
    memory::init(multiboot_info as usize);
    scheduler::init();
    filesystem::init();
    networking::init();
    ipc::init();
    syscalls::init();

    unsafe { irq::enable() };

    klog!(LogLevel::Info, "interrupts enabled");

    let mut last_tick = 0;
    loop {
        let ticks = timer::uptime_ticks();
        if ticks != last_tick && ticks % 100 == 0 {
            klog!(LogLevel::Trace, "uptime ticks: {}", ticks);
            scheduler::dump();
            last_tick = ticks;
        }
        x86::halt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    klog!(LogLevel::Error, "KERNEL PANIC: {}", info);
    loop {
        x86::halt();
    }
}
