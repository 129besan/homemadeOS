use crate::mm::addr::{PhysAddr, VirtAddr, PhysFrame};
use crate::mm::paging::flags::PageFlags;
use crate::mm::frame_allocator::FrameAllocator;

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

pub struct PageTableWalker<'a> {
    level: usize,
    tables: [&'a PageTable; 4],
}

impl<'a> PageTableWalker<'a> {
    pub fn new(pml4: &'a PageTable) -> Self {
        PageTableWalker {
            level: 0,
            tables: [pml4, pml4, pml4, pml4],
        }
    }

    pub fn walk(&mut self, virt: VirtAddr) -> Result<&mut PageTableEntry, ()> {
        let indices = virt_indices(virt);
        for level in 0..4 {
            let table = &self.tables[level];
            let entry = unsafe {
                &mut *(&table.entries[indices[level]] as *const PageTableEntry as *mut PageTableEntry)
            };
            if level == 3 {
                return Ok(entry);
            }
            if !entry.is_present() {
                return Err(());
            }
            let next_table = unsafe { &mut *(entry.addr().0 as *mut PageTable) };
            self.tables[level + 1] = next_table;
        }
        Err(())
    }
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
            entry.set_addr(frame.start_addr(), PageFlags::PRESENT | PageFlags::WRITABLE);
        }
        let next = entry.addr();
        table = unsafe { &mut *(next.0 as *mut PageTable) };
    }

    let entry = &mut table.entries[indices[3] as usize];
    entry.set_addr(phys, flags | PageFlags::PRESENT);
    Ok(())
}

pub fn unmap_page(pml4: &mut PageTable, virt: VirtAddr) -> Result<PhysAddr, ()> {
    let mut walker = PageTableWalker::new(pml4);
    let entry = walker.walk(virt)?;
    if !entry.is_present() {
        return Err(());
    }
    let phys = entry.addr();
    entry.0 = 0;
    unsafe { core::arch::asm!("invlpg ({0})", in(reg) virt.0, options(nostack, preserves_flags)) };
    Ok(phys)
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
