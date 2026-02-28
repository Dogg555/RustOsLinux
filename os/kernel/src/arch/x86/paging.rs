use core::arch::asm;

#[repr(align(4096))]
struct PageDirectory([u32; 1024]);

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory([0; 1024]);

pub fn init_identity_4mb() {
    unsafe {
        PAGE_DIRECTORY.0[0] = 0x00000083;
        let pd_addr = &PAGE_DIRECTORY as *const _ as u32;
        asm!("mov cr3, {0:e}", in(reg) pd_addr, options(nostack, preserves_flags));
        asm!(
            "mov eax, cr4",
            "or eax, 0x10",
            "mov cr4, eax",
            out("eax") _,
            options(nostack)
        );
        asm!(
            "mov eax, cr0",
            "or eax, 0x80000000",
            "mov cr0, eax",
            out("eax") _,
            options(nostack)
        );
    }
}
