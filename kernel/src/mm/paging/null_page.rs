use crate::mm::addr::VirtAddr;
use crate::mm::paging::page_table::{PageTable, virt_indices};

pub fn protect_null(pml4: &mut PageTable) {
    let indices = virt_indices(VirtAddr(0));

    let mut table = pml4;
    for &idx in &indices[..3] {
        let entry = &mut table.entries[idx];
        if !entry.is_present() {
            return;
        }
        let next = entry.addr().0;
        table = unsafe { &mut *(next as *mut PageTable) };
    }
    table.entries[indices[3]].0 = 0;
}
