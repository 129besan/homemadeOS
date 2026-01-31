use crate::mm::addr::{PhysAddr, VirtAddr};
use crate::mm::paging::flags::PageFlags;

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn new() -> Self {
        PageTableEntry(0)
    }

    pub fn set_addr(&mut self, addr: PhysAddr, flags: PageFlags) {
        self.0 = (addr.0 & 0x000f_ffff_ffff_f000) | flags.bits();
    }

    pub fn addr(&self) -> PhysAddr {
        PhysAddr(self.0 & 0x000f_ffff_ffff_f000)
    }

    pub fn flags(&self) -> PageFlags {
        PageFlags::from_bits_truncate(self.0)
    }

    pub fn is_present(&self) -> bool {
        self.0 & 1 != 0
    }
}

pub fn walk_page_table(pml4: &PageTable, virt: VirtAddr) -> Option<PhysAddr> {
    let indices = virt_indices(virt);
    let mut table = pml4;

    for &level in &indices {
        let entry = &table.entries[level as usize];
        if !entry.is_present() {
            return None;
        }
        let next = entry.addr();
        table = unsafe { &*(next.0 as *const PageTable) };
    }

    Some(PhysAddr(virt.0))
}

pub fn virt_indices(virt: VirtAddr) -> [usize; 4] {
    let v = virt.0;
    [
        ((v >> 39) & 0x1ff) as usize,
        ((v >> 30) & 0x1ff) as usize,
        ((v >> 21) & 0x1ff) as usize,
        ((v >> 12) & 0x1ff) as usize,
    ]
}
