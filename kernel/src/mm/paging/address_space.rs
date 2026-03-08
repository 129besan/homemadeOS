use crate::mm::addr::VirtAddr;
use crate::mm::paging::flags::PageFlags;
use crate::mm::paging::page_table::{PageTableEntry, PageTable};
use crate::mm::frame_allocator::FRAME_ALLOCATOR;
use crate::sync::spinlock::SpinLock;

pub struct AddressSpace {
    pml4: *mut PageTable,
}

impl AddressSpace {
    pub fn new_kernel() -> Self {
        let pml4 = active_pml4();
        AddressSpace { pml4 }
    }

    pub fn new_user() -> Self {
        let mut allocator = FRAME_ALLOCATOR.lock();
        let frame = allocator.alloc().expect("no frame for user PML4");
        let pml4 = frame.as_mut_ptr::<PageTable>();

        let active_pml4 = active_pml4();
        unsafe {
            for i in 256..512 {
                (*pml4)[i] = (*active_pml4)[i];
            }
        }

        AddressSpace { pml4 }
    }
}

fn active_pml4() -> *mut PageTable {
    let cr3: u64;
    unsafe { core::arch::asm!("mov {}, cr3", out(reg) cr3); }
    (cr3 & !0xfff) as *mut PageTable
}
