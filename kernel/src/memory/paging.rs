use super::{frame_allocator::FrameAllocator, PhysicalAddress, VirtualAddress};

const ENTRY_COUNT: usize = 512;

const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const HUGE_PAGE: u64 = 1 << 7;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    const fn unused() -> Self {
        Self(0)
    }

    fn set_addr(&mut self, addr: PhysicalAddress, flags: u64) {
        self.0 = (addr.as_u64() & 0x000f_ffff_ffff_f000) | flags;
    }

    fn is_present(self) -> bool {
        (self.0 & PRESENT) != 0
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    fn zero(&mut self) {
        self.entries.fill(PageTableEntry::unused());
    }
}

pub struct PageTables {
    pml4: PageTable,
    pdpt: PageTable,
    pd: PageTable,
}

impl PageTables {
    pub const fn new() -> Self {
        Self {
            pml4: PageTable {
                entries: [PageTableEntry::unused(); ENTRY_COUNT],
            },
            pdpt: PageTable {
                entries: [PageTableEntry::unused(); ENTRY_COUNT],
            },
            pd: PageTable {
                entries: [PageTableEntry::unused(); ENTRY_COUNT],
            },
        }
    }

    pub fn setup_identity_map(&mut self, length: u64) {
        self.pml4.zero();
        self.pdpt.zero();
        self.pd.zero();

        let pml4_addr = PhysicalAddress::new((&self.pdpt as *const PageTable) as u64);
        let pdpt_addr = PhysicalAddress::new((&self.pd as *const PageTable) as u64);

        self.pml4.entries[0].set_addr(pml4_addr, PRESENT | WRITABLE);
        self.pdpt.entries[0].set_addr(pdpt_addr, PRESENT | WRITABLE);

        map_2m_range(&mut self.pd, 0, length);
    }

    pub fn pml4(&self) -> &PageTable {
        &self.pml4
    }
}

pub fn map_kernel_higher_half(
    tables: &mut PageTables,
    frame_allocator: &mut FrameAllocator<'_>,
    virt_start: VirtualAddress,
    phys_start: PhysicalAddress,
    length: u64,
) -> Result<(), &'static str> {
    let _bootstrap = frame_allocator
        .allocate_frame()
        .ok_or("no usable frame for paging bootstrap")?;

    tables.pml4.zero();
    tables.pdpt.zero();
    tables.pd.zero();

    let pdpt_addr = PhysicalAddress::new((&tables.pdpt as *const PageTable) as u64);
    let pd_addr = PhysicalAddress::new((&tables.pd as *const PageTable) as u64);

    tables.pml4.entries[virt_start.pml4_index()].set_addr(pdpt_addr, PRESENT | WRITABLE);
    tables.pdpt.entries[virt_start.pdpt_index()].set_addr(pd_addr, PRESENT | WRITABLE);

    map_2m_range_from(
        &mut tables.pd,
        virt_start.pd_index(),
        phys_start.as_u64(),
        length,
    );

    Ok(())
}

fn map_2m_range(pd: &mut PageTable, start_phys: u64, length: u64) {
    map_2m_range_from(pd, 0, start_phys, length)
}

fn map_2m_range_from(pd: &mut PageTable, pd_index_start: usize, start_phys: u64, length: u64) {
    let huge_page_size: u64 = 2 * 1024 * 1024;
    let pages = length.div_ceil(huge_page_size);

    for i in 0..pages as usize {
        let phys = PhysicalAddress::new(start_phys + (i as u64 * huge_page_size));
        pd.entries[pd_index_start + i].set_addr(phys, PRESENT | WRITABLE | HUGE_PAGE);
    }
}

pub fn enable_paging(_pml4: &PageTable) {
    // Intentionally isolated for architecture bring-up.
    // Real CR3/CR0 updates will be wired when low-level CPU init lands.
}

#[cfg(test)]
mod tests {
    use bootloader::{MemoryRegion, MemoryRegionKind};

    use super::*;

    #[test]
    fn higher_half_mapping_sets_present_entries() {
        let mut tables = PageTables::new();
        let regions = [MemoryRegion {
            start: 0x1000,
            end: 0x20000,
            kind: MemoryRegionKind::Usable,
        }];
        let mut allocator = FrameAllocator::new(&regions);

        map_kernel_higher_half(
            &mut tables,
            &mut allocator,
            VirtualAddress::new(0xffff_8000_0000_0000),
            PhysicalAddress::new(0x200000),
            4 * 1024 * 1024,
        )
        .unwrap();

        assert!(tables.pml4.entries[256].is_present());
        assert!(tables.pdpt.entries[0].is_present());
        assert!(tables.pd.entries[0].is_present());
        assert!(tables.pd.entries[1].is_present());
    }
}
