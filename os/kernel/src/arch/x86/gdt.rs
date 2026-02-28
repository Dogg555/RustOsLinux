use core::arch::asm;

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    const fn empty() -> Self {
        Self { limit_low: 0, base_low: 0, base_mid: 0, access: 0, granularity: 0, base_high: 0 }
    }

    const fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (granularity & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

static mut GDT: [GdtEntry; 3] = [
    GdtEntry::empty(),
    GdtEntry::new(0, 0xFFFFF, 0x9A, 0xCF),
    GdtEntry::new(0, 0xFFFFF, 0x92, 0xCF),
];

pub fn init() {
    let gdtr = GdtDescriptor {
        limit: (core::mem::size_of::<[GdtEntry; 3]>() - 1) as u16,
        base: unsafe { &GDT as *const _ as u32 },
    };

    unsafe {
        asm!("lgdt [{0}]", in(reg) &gdtr, options(readonly, nostack, preserves_flags));
        asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "jmp 0x08:2f",
            "2:",
            options(nostack)
        );
    }
}
