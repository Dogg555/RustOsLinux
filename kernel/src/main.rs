#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod memory;

use core::panic::PanicInfo;
use memory::{
    allocator, frame_allocator::FrameAllocator, page_fault, paging, PhysicalAddress, VirtualAddress,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Temporary boot contract stub until real bootloader handoff is wired.
    // This keeps phase-3 memory initialization deterministic in tests and early bring-up.
    let boot_info = bootloader::BootInfo::empty();

    memory::init(&boot_info);

    // Exercise allocation path to ensure global allocator is alive.
    let _vec = alloc::vec![1_u64, 2, 3, 4];

    // Exercise page mapping API surface.
    let mut allocator = FrameAllocator::new(boot_info.memory_regions());
    let mut tables = paging::PageTables::new();
    tables.setup_identity_map(4 * 1024 * 1024);
    paging::enable_paging(tables.pml4());

    let _ = paging::map_kernel_higher_half(
        &mut tables,
        &mut allocator,
        VirtualAddress::new(0xffff_8000_0000_0000),
        PhysicalAddress::new(0x0010_0000),
        2 * 1024 * 1024,
    );

    // Touch page-fault diagnostics path without faulting real memory.
    page_fault::record_fault(0xdead_beef, 0b10);
    let _ = page_fault::last_fault();

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    allocator::record_alloc_error(layout.size(), layout.align());
    loop {
        core::hint::spin_loop();
    }
}
