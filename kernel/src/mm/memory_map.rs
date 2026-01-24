use crate::BootInfo;
use crate::mm::addr::PhysAddr;

#[repr(u32)]
pub enum MemoryRegionType {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadMemory = 5,
    BootloaderReclaimable = 6,
    Kernel = 7,
    Framebuffer = 8,
}

#[repr(C)]
pub struct MemoryRegion {
    pub start: u64,
    pub length: u64,
    pub region_type: u32,
}

pub fn parse_memory_map(boot_info: &BootInfo) -> &'static [MemoryRegion] {
    let ptr = boot_info.memory_map_ptr as *const MemoryRegion;
    let len = boot_info.memory_map_len as usize;
    if ptr.is_null() || len == 0 {
        return &[];
    }
    unsafe { core::slice::from_raw_parts(ptr, len) }
}
