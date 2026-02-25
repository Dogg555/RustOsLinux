#![no_std]

/// BootInfo is the hand-off contract between bootloader and kernel.
/// It will be expanded as framebuffer, memory map, and ACPI info are wired.
#[repr(C)]
pub struct BootInfo {
    pub framebuffer_base: u64,
    pub framebuffer_size: u64,
    pub memory_map_ptr: u64,
    pub memory_map_len: u64,
}
