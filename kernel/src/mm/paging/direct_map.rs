use crate::mm::addr::{PhysAddr, VirtAddr, PAGE_SIZE};
use crate::mm::frame_allocator::FrameAllocator;
use crate::mm::paging::flags::PageFlags;
use crate::mm::paging::page_table::{map_page, PageTable};

pub const DIRECT_MAP_BASE: u64 = 0xffff_8000_0000_0000;

pub fn create_direct_map(
    pml4: &mut PageTable,
    phys_end: u64,
    allocator: &mut FrameAllocator,
) {
    let num_pages = (phys_end as usize + PAGE_SIZE as usize - 1) / PAGE_SIZE as usize;
    for i in 0..num_pages {
        let phys = PhysAddr(i as u64 * PAGE_SIZE);
        let virt = VirtAddr(DIRECT_MAP_BASE + i as u64 * PAGE_SIZE);
        let flags = PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::NO_EXECUTE;
        let _ = map_page(pml4, virt, phys, flags, allocator);
    }
}
