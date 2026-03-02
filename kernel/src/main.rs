#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use kernel::memory::{
    allocator, frame_allocator::FrameAllocator, page_fault, paging, PhysicalAddress, VirtualAddress,
};
use kernel::scheduler::{RegisterState, RoundRobinScheduler, Task};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Temporary boot contract stub until real bootloader handoff is wired.
    // This keeps phase-3 memory initialization deterministic in tests and early bring-up.
    let boot_info = bootloader::BootInfo::empty();

    kernel::memory::init(&boot_info);

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

    // Phase 4: timer + scheduler baseline.
    let _pit = kernel::timer::init(100);
    let mut scheduler = RoundRobinScheduler::new();
    let _ = scheduler.add_task(Task::new(0x1000, 0x8000));
    let _ = scheduler.add_task(Task::new(0x2000, 0x9000));

    let mut registers = RegisterState::empty();
    registers.rax = 1;
    registers.rsp = 0x8000;
    scheduler.save_current_registers(registers);

    let _tick = kernel::timer::handle_timer_interrupt();
    let _switch = scheduler.on_timer_tick();
    let _next_regs = scheduler.load_next_registers();

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
