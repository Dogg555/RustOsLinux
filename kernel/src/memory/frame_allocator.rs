use bootloader::{MemoryRegion, MemoryRegionKind};

use super::{PhysicalAddress, FRAME_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysFrame {
    start: PhysicalAddress,
}

impl PhysFrame {
    pub const fn from_start_address(addr: PhysicalAddress) -> Self {
        Self {
            start: addr.align_down(),
        }
    }

    pub const fn start_address(self) -> PhysicalAddress {
        self.start
    }
}

pub struct FrameAllocator<'a> {
    regions: &'a [MemoryRegion],
    region_index: usize,
    next: u64,
}

impl<'a> FrameAllocator<'a> {
    pub fn new(regions: &'a [MemoryRegion]) -> Self {
        let mut allocator = Self {
            regions,
            region_index: 0,
            next: 0,
        };
        allocator.advance_to_usable_region();
        allocator
    }

    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {
        loop {
            let region = self.regions.get(self.region_index)?;
            let aligned_start = align_up(region.start, FRAME_SIZE);
            if self.next < aligned_start {
                self.next = aligned_start;
            }

            if self.next + FRAME_SIZE <= region.end {
                let frame = PhysFrame::from_start_address(PhysicalAddress::new(self.next));
                self.next += FRAME_SIZE;
                return Some(frame);
            }

            self.region_index += 1;
            self.advance_to_usable_region();
        }
    }

    fn advance_to_usable_region(&mut self) {
        while let Some(region) = self.regions.get(self.region_index) {
            if region.kind == MemoryRegionKind::Usable && region.start < region.end {
                self.next = align_up(region.start, FRAME_SIZE);
                return;
            }
            self.region_index += 1;
        }
    }
}

const fn align_up(value: u64, align: u64) -> u64 {
    (value + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(start: u64, end: u64, kind: MemoryRegionKind) -> MemoryRegion {
        MemoryRegion { start, end, kind }
    }

    #[test]
    fn skips_reserved_and_allocates_usable_frames() {
        let regions = [
            region(0, 0x2000, MemoryRegionKind::Reserved),
            region(0x2000, 0x5000, MemoryRegionKind::Usable),
            region(0x8000, 0x9000, MemoryRegionKind::Usable),
        ];

        let mut allocator = FrameAllocator::new(&regions);

        assert_eq!(
            allocator.allocate_frame().unwrap().start_address().as_u64(),
            0x2000
        );
        assert_eq!(
            allocator.allocate_frame().unwrap().start_address().as_u64(),
            0x3000
        );
        assert_eq!(
            allocator.allocate_frame().unwrap().start_address().as_u64(),
            0x4000
        );
        assert_eq!(
            allocator.allocate_frame().unwrap().start_address().as_u64(),
            0x8000
        );
        assert!(allocator.allocate_frame().is_none());
    }
}
