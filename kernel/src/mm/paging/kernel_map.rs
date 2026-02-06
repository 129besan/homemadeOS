use crate::mm::addr::{PhysAddr, VirtAddr};
use crate::mm::frame_allocator::FrameAllocator;
use crate::mm::paging::flags::PageFlags;
use crate::mm::paging::page_table::{map_page, PageTable};

pub fn map_kernel_sections(
    pml4: &mut PageTable,
    kernel_phys_start: u64,
    kernel_phys_end: u64,
    allocator: &mut FrameAllocator,
) {
    let text_start = kernel_phys_start;
    let text_end = text_start + 0x10000;
    map_range(pml4, text_start, text_end, PageFlags::PRESENT, allocator);

    let rodata_start = text_end;
    let rodata_end = rodata_start + 0x10000;
    map_range(pml4, rodata_start, rodata_end, PageFlags::PRESENT | PageFlags::NO_EXECUTE, allocator);

    let data_start = rodata_end;
    let data_end = kernel_phys_end;
    map_range(pml4, data_start, data_end, PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::NO_EXECUTE, allocator);
}

fn map_range(
    pml4: &mut PageTable,
    phys_start: u64,
    phys_end: u64,
    flags: PageFlags,
    allocator: &mut FrameAllocator,
) {
    let mut addr = phys_start;
    while addr < phys_end {
        let virt = VirtAddr(addr + 0xffffffff80000000);
        let phys = PhysAddr(addr);
        let _ = map_page(pml4, virt, phys, flags, allocator);
        addr += 4096;
    }
}
