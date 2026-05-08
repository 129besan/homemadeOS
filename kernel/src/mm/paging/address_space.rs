use crate::mm::addr::{VirtAddr, PhysAddr, PAGE_SIZE};
use crate::mm::paging::flags::PageFlags;
use crate::mm::paging::page_table::{PageTable, map_page};
use crate::mm::frame_allocator::FRAME_ALLOCATOR;

pub const USER_STACK_PAGES: u64 = 4;
pub const USER_STACK_TOP: u64 = 0x0000_7fff_ffff_e000;

pub struct AddressSpace {
    pml4: *mut PageTable,
}

impl AddressSpace {
    pub fn new_kernel() -> Self {
        let pml4 = active_pml4();
        AddressSpace { pml4 }
    }

    pub fn new_user() -> Self {
        let mut alloc_guard = FRAME_ALLOCATOR.lock();
        let allocator = alloc_guard.as_mut().expect("frame allocator not initialized");
        let frame = allocator.alloc().expect("no frame for user PML4");
        let pml4 = frame.start_addr().0 as *mut PageTable;

        let active_pml4 = active_pml4();
        unsafe {
            for i in 0..512 {
                (*pml4).entries[i].0 = 0;
            }
            for i in 256..512 {
                (*pml4).entries[i] = (*active_pml4).entries[i];
            }
        }

        drop(alloc_guard);
        AddressSpace { pml4 }
    }

    pub fn map_user_stack(&mut self) -> VirtAddr {
        let stack_base = USER_STACK_TOP - USER_STACK_PAGES * PAGE_SIZE;
        let mut alloc_guard = FRAME_ALLOCATOR.lock();
        let allocator = alloc_guard.as_mut().expect("frame allocator not initialized");
        for i in 0..USER_STACK_PAGES {
            let frame = allocator.alloc().expect("out of memory for user stack");
            let virt = VirtAddr(stack_base + i * PAGE_SIZE);
            unsafe {
                let pml4 = &mut *self.pml4;
                map_page(pml4, virt, frame.start_addr(), PageFlags::USER | PageFlags::WRITABLE | PageFlags::PRESENT, allocator).unwrap();
            }
        }
        VirtAddr(USER_STACK_TOP)
    }

    pub fn map_user_page(&mut self, virt: VirtAddr, flags: PageFlags) -> Result<PhysAddr, ()> {
        let mut alloc_guard = FRAME_ALLOCATOR.lock();
        let allocator = alloc_guard.as_mut().ok_or(())?;
        let frame = allocator.alloc().ok_or(())?;
        let phys = frame.start_addr();

        unsafe {
            core::ptr::write_bytes(phys.0 as *mut u8, 0, PAGE_SIZE as usize);
            let pml4 = &mut *self.pml4;
            map_page(
                pml4,
                virt,
                phys,
                flags | PageFlags::USER | PageFlags::PRESENT,
                allocator,
            )?;
        }

        Ok(phys)
    }
}

fn active_pml4() -> *mut PageTable {
    let cr3: u64;
    unsafe { core::arch::asm!("mov {}, cr3", out(reg) cr3); }
    (cr3 & !0xfff) as *mut PageTable
}
