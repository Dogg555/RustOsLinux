pub mod allocator;
pub mod frame_allocator;
pub mod page_fault;
pub mod paging;

use bootloader::BootInfo;

pub const FRAME_SIZE: u64 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub const fn align_down(self) -> Self {
        Self(self.0 & !(FRAME_SIZE - 1))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub const fn page_offset(self) -> u64 {
        self.0 & 0xfff
    }

    pub const fn pml4_index(self) -> usize {
        ((self.0 >> 39) & 0x1ff) as usize
    }

    pub const fn pdpt_index(self) -> usize {
        ((self.0 >> 30) & 0x1ff) as usize
    }

    pub const fn pd_index(self) -> usize {
        ((self.0 >> 21) & 0x1ff) as usize
    }

    pub const fn pt_index(self) -> usize {
        ((self.0 >> 12) & 0x1ff) as usize
    }
}

pub fn init(boot_info: &BootInfo) {
    allocator::init();
    page_fault::init();

    // Initialize and sanity-check frame allocator with boot memory map.
    let mut frames = frame_allocator::FrameAllocator::new(boot_info.memory_regions());
    let _ = frames.allocate_frame();
}

#[cfg(test)]
mod tests {
    use super::VirtualAddress;

    #[test]
    fn virtual_address_indices_are_extracted_correctly() {
        let vaddr = VirtualAddress::new(0xffff_8123_4567_89ab);
        assert_eq!(vaddr.pml4_index(), 258);
        assert_eq!(vaddr.pdpt_index(), 141);
        assert_eq!(vaddr.pd_index(), 43);
        assert_eq!(vaddr.pt_index(), 120);
        assert_eq!(vaddr.page_offset(), 0x9ab);
    }
}
