pub mod address_space;
pub mod direct_map;
pub mod flags;
pub mod kernel_map;
pub mod null_page;
pub mod page_table;
pub mod tlb;

use crate::mm::addr::{VirtAddr, PAGE_SIZE};
use crate::mm::frame_allocator::FRAME_ALLOCATOR;
use crate::mm::heap::{HEAP_START, HEAP_SIZE};
use crate::mm::paging::flags::PageFlags;
use crate::mm::paging::page_table::{map_page, PageTable};

pub unsafe fn init_heap_paging() {
    let cr3: u64;
    core::arch::asm!("mov {}, cr3", out(reg) cr3);
    let pml4 = &mut *((cr3 & !0xfff) as *mut PageTable);

    let mut alloc_guard = FRAME_ALLOCATOR.lock();
    let allocator = alloc_guard.as_mut().expect("frame allocator not initialized");

    let heap_pages = HEAP_SIZE / PAGE_SIZE as usize;
    for i in 0..heap_pages {
        let frame = allocator.alloc().expect("out of memory for heap");
        let virt = VirtAddr(HEAP_START as u64 + i as u64 * PAGE_SIZE);
        let _ = map_page(
            pml4,
            virt,
            frame.start_addr(),
            PageFlags::PRESENT | PageFlags::WRITABLE,
            allocator,
        );
    }

    drop(alloc_guard);
    crate::log_info!("heap paging initialized ({} pages)", heap_pages);
}
