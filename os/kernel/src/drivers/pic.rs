use x86::io::{inb, outb};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_COMMAND: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_COMMAND: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;

pub fn init() {
    unsafe {
        let a1 = inb(PIC1_DATA);
        let a2 = inb(PIC2_DATA);

        outb(PIC1_COMMAND, 0x11);
        outb(PIC2_COMMAND, 0x11);

        outb(PIC1_DATA, 0x20);
        outb(PIC2_DATA, 0x28);

        outb(PIC1_DATA, 4);
        outb(PIC2_DATA, 2);

        outb(PIC1_DATA, 0x01);
        outb(PIC2_DATA, 0x01);

        outb(PIC1_DATA, a1 & !0x03);
        outb(PIC2_DATA, a2);
    }
}

pub fn end_of_interrupt(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, 0x20);
        }
        outb(PIC1_COMMAND, 0x20);
    }
}
