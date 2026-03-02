use linked_list_allocator::LockedHeap;
use multiboot2::{BootInformation, MemoryAreaType};

use crate::arch::x86::paging;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const HEAP_SIZE: usize = 1024 * 64;
#[repr(align(16))]
struct Heap([u8; HEAP_SIZE]);
static mut HEAP: Heap = Heap([0; HEAP_SIZE]);

pub fn init(multiboot_info_addr: usize) {
    paging::init_identity_4mb();
    unsafe {
        ALLOCATOR.lock().init(HEAP.0.as_ptr() as usize, HEAP_SIZE);
    }

    let boot_info = unsafe { BootInformation::load(multiboot_info_addr as *const ()) }
        .expect("valid multiboot2 info");

    println!("Memory map:");
    if let Some(map) = boot_info.memory_map_tag() {
        for area in map.memory_areas() {
            if area.typ() == MemoryAreaType::Available {
                println!("  avail {:08x}-{:08x}", area.start_address(), area.end_address());
            }
        }
    }
}
