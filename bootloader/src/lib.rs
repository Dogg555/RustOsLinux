#![no_std]

/// Maximum number of memory regions that can be handed to the kernel in the
/// static [`BootInfo`] array.
pub const MAX_MEMORY_REGIONS: usize = 128;

/// Framebuffer metadata handed from the bootloader to the kernel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct FramebufferInfo {
    pub base: u64,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bytes_per_pixel: u32,
}

impl FramebufferInfo {
    /// Returns true when the framebuffer descriptor looks coherent.
    pub const fn is_valid(self) -> bool {
        self.base != 0
            && self.size != 0
            && self.width != 0
            && self.height != 0
            && self.stride >= self.width
            && self.bytes_per_pixel != 0
    }
}

/// Kind of a memory region found by the bootloader.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoryRegionKind {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    Mmio = 5,
}

/// One physical memory-map region passed to the kernel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub kind: MemoryRegionKind,
}

impl MemoryRegion {
    /// Returns true when this region has a non-empty and ordered span.
    pub const fn is_valid(self) -> bool {
        self.start < self.end
    }
}

/// BootInfo is the hand-off contract between bootloader and kernel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct BootInfo {
    pub framebuffer: FramebufferInfo,
    pub memory_regions_len: u32,
    pub _reserved: u32,
    pub memory_regions: [MemoryRegion; MAX_MEMORY_REGIONS],
}

impl BootInfo {
    /// Creates an empty boot contract.
    pub const fn empty() -> Self {
        Self {
            framebuffer: FramebufferInfo {
                base: 0,
                size: 0,
                width: 0,
                height: 0,
                stride: 0,
                bytes_per_pixel: 0,
            },
            memory_regions_len: 0,
            _reserved: 0,
            memory_regions: [MemoryRegion {
                start: 0,
                end: 0,
                kind: MemoryRegionKind::Reserved,
            }; MAX_MEMORY_REGIONS],
        }
    }

    /// Number of valid memory regions currently stored.
    pub const fn region_count(&self) -> usize {
        self.memory_regions_len as usize
    }

    /// Returns a region slice that contains only populated entries.
    pub fn memory_regions(&self) -> &[MemoryRegion] {
        let len = self.region_count();
        if len > MAX_MEMORY_REGIONS {
            return &self.memory_regions[..0];
        }
        &self.memory_regions[..len]
    }

    /// Validates basic handoff invariants before entering kernel code.
    pub fn validate(&self) -> Result<(), BootError> {
        let len = self.region_count();
        if len > MAX_MEMORY_REGIONS {
            return Err(BootError::RegionCountOverflow {
                count: len,
                max: MAX_MEMORY_REGIONS,
            });
        }

        if !self.framebuffer.is_valid() {
            return Err(BootError::InvalidFramebuffer);
        }

        let mut i = 0;
        while i < len {
            if !self.memory_regions[i].is_valid() {
                return Err(BootError::InvalidMemoryRegion { index: i });
            }
            i += 1;
        }

        Ok(())
    }
}

impl Default for BootInfo {
    fn default() -> Self {
        Self::empty()
    }
}

/// Boot contract validation failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootError {
    InvalidFramebuffer,
    InvalidMemoryRegion { index: usize },
    RegionCountOverflow { count: usize, max: usize },
}

/// Mutable builder used by the bootloader while probing platform data.
#[derive(Debug, Clone, Copy)]
pub struct BootInfoBuilder {
    boot_info: BootInfo,
}

impl BootInfoBuilder {
    pub const fn new() -> Self {
        Self {
            boot_info: BootInfo::empty(),
        }
    }

    pub fn with_framebuffer(mut self, framebuffer: FramebufferInfo) -> Self {
        self.boot_info.framebuffer = framebuffer;
        self
    }

    pub fn push_memory_region(mut self, region: MemoryRegion) -> Result<Self, BootError> {
        let len = self.boot_info.region_count();
        if len >= MAX_MEMORY_REGIONS {
            return Err(BootError::RegionCountOverflow {
                count: len + 1,
                max: MAX_MEMORY_REGIONS,
            });
        }

        self.boot_info.memory_regions[len] = region;
        self.boot_info.memory_regions_len += 1;
        Ok(self)
    }

    pub fn build(self) -> Result<BootInfo, BootError> {
        self.boot_info.validate()?;
        Ok(self.boot_info)
    }
}

impl Default for BootInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_framebuffer() -> FramebufferInfo {
        FramebufferInfo {
            base: 0x1000,
            size: 1024 * 768 * 4,
            width: 1024,
            height: 768,
            stride: 1024,
            bytes_per_pixel: 4,
        }
    }

    #[test]
    fn builder_rejects_missing_framebuffer() {
        let result = BootInfoBuilder::new().build();
        assert_eq!(result, Err(BootError::InvalidFramebuffer));
    }

    #[test]
    fn builder_rejects_invalid_region() {
        let result = BootInfoBuilder::new()
            .with_framebuffer(valid_framebuffer())
            .push_memory_region(MemoryRegion {
                start: 0x3000,
                end: 0x2000,
                kind: MemoryRegionKind::Usable,
            })
            .expect("capacity should be available")
            .build();

        assert_eq!(result, Err(BootError::InvalidMemoryRegion { index: 0 }));
    }

    #[test]
    fn builder_accepts_valid_boot_info() {
        let boot_info = BootInfoBuilder::new()
            .with_framebuffer(valid_framebuffer())
            .push_memory_region(MemoryRegion {
                start: 0,
                end: 0x9f000,
                kind: MemoryRegionKind::Usable,
            })
            .expect("region should fit")
            .push_memory_region(MemoryRegion {
                start: 0xf0000,
                end: 0x100000,
                kind: MemoryRegionKind::Reserved,
            })
            .expect("region should fit")
            .build()
            .expect("boot contract should validate");

        assert_eq!(boot_info.region_count(), 2);
        assert_eq!(boot_info.memory_regions()[0].kind, MemoryRegionKind::Usable);
        assert_eq!(
            boot_info.memory_regions()[1].kind,
            MemoryRegionKind::Reserved
        );
    }
}
