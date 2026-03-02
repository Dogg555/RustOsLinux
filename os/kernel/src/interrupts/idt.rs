use core::arch::asm;

use super::handlers;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    offset_high: u16,
}

impl IdtEntry {
    const fn missing() -> Self {
        Self { offset_low: 0, selector: 0, zero: 0, flags: 0, offset_high: 0 }
    }

    fn new(handler: usize, flags: u8) -> Self {
        Self {
            offset_low: (handler & 0xFFFF) as u16,
            selector: 0x08,
            zero: 0,
            flags,
            offset_high: ((handler >> 16) & 0xFFFF) as u16,
        }
    }
}

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u32,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

pub fn init() {
    unsafe {
        IDT[32] = IdtEntry::new(handlers::timer_interrupt as usize, 0x8E);
        IDT[33] = IdtEntry::new(handlers::keyboard_interrupt as usize, 0x8E);
        IDT[14] = IdtEntry::new(handlers::page_fault_handler as usize, 0x8E);
        IDT[8] = IdtEntry::new(handlers::double_fault_handler as usize, 0x8E);

        let idtr = Idtr {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: &IDT as *const _ as u32,
        };

        asm!("lidt [{0}]", in(reg) &idtr, options(readonly, nostack, preserves_flags));
    }
}
