use crate::mm::addr::{PhysAddr, VirtAddr, PhysFrame};
use crate::mm::paging::flags::PageFlags;
use crate::mm::frame_allocator::FrameAllocator;

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(pub(crate) u64);

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

pub fn unmap_page(pml4: &mut PageTable, virt: VirtAddr) -> Result<PhysAddr, ()> {
    let indices = virt_indices(virt);
    let mut table = pml4;
    for &level in &indices[..3] {
        let entry = &table.entries[level];
        if !entry.is_present() {
            return Err(());
        }
        let next = entry.addr();
        table = unsafe { &mut *(next.0 as *mut PageTable) };
    }
    let entry = &mut table.entries[indices[3]];
    if !entry.is_present() {
        return Err(());
    }
    let phys = entry.addr();
    entry.0 = 0;
    unsafe { core::arch::asm!("invlpg ({0})", in(reg) virt.0, options(nostack, preserves_flags)) };
    Ok(phys)
}

pub fn map_page(
    pml4: &mut PageTable,
    virt: VirtAddr,
    phys: PhysAddr,
    flags: PageFlags,
    allocator: &mut FrameAllocator,
) -> Result<(), ()> {
    let indices = virt_indices(virt);
    let mut table = pml4;

    for &level in &indices[..3] {
        let entry = &mut table.entries[level as usize];
        if !entry.is_present() {
            let frame = allocator.alloc().ok_or(())?;
            let new_table = unsafe { &mut *(frame.start_addr().0 as *mut PageTable) };
            for e in new_table.entries.iter_mut() {
                e.0 = 0;
            }
            let mut table_flags = PageFlags::PRESENT | PageFlags::WRITABLE;
            if flags.contains(PageFlags::USER) {
                table_flags |= PageFlags::USER;
            }
            entry.set_addr(frame.start_addr(), table_flags);
        }
        let next = entry.addr();
        table = unsafe { &mut *(next.0 as *mut PageTable) };
    }

    let entry = &mut table.entries[indices[3] as usize];
    entry.set_addr(phys, flags | PageFlags::PRESENT);
    Ok(())
}

pub fn translate(pml4: &PageTable, virt: VirtAddr) -> Option<PhysAddr> {
    let indices = virt_indices(virt);
    let mut table = pml4;
    let offset = virt.0 & 0xfff;

    for (level, &i) in indices.iter().enumerate() {
        let entry = &table.entries[i as usize];
        if !entry.is_present() {
            return None;
        }
        if level == 3 {
            let phys = entry.addr().0 | offset;
            return Some(PhysAddr(phys));
        }
        let next = entry.addr();
        table = unsafe { &*(next.0 as *const PageTable) };
    }

    None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virt_indices() {
        let v = VirtAddr(0xffff_8000_0000_0000);
        let idx = virt_indices(v);
        assert_eq!(idx[0], 0x1ff);
        assert_eq!(idx[1], 0x000);
        assert_eq!(idx[2], 0x000);
        assert_eq!(idx[3], 0x000);
    }

    #[test]
    fn test_page_table_entry() {
        let mut entry = PageTableEntry::new();
        assert!(!entry.is_present());
        entry.set_addr(PhysAddr(0x1000), PageFlags::PRESENT | PageFlags::WRITABLE);
        assert!(entry.is_present());
        assert_eq!(entry.addr(), PhysAddr(0x1000));
        assert!(entry.flags().contains(PageFlags::WRITABLE));
    }

    #[test]
    fn test_translate_not_mapped() {
        let mut table = PageTable { entries: [PageTableEntry(0); 512] };
        let v = VirtAddr(0x1000);
        assert!(translate(&table, v).is_none());
    }
}
