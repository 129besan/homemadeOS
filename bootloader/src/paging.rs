pub const PAGE_SIZE: u64 = 4096;
pub const PAGE_TABLE_ENTRIES: usize = 512;

pub struct PageTable {
    entries: [u64; PAGE_TABLE_ENTRIES],
}

impl PageTable {
    pub const fn zeroed() -> Self {
        PageTable { entries: [0; PAGE_TABLE_ENTRIES] }
    }

    fn set_entry(&mut self, index: usize, addr: u64, flags: u64) {
        self.entries[index] = (addr & 0x000f_ffff_ffff_f000) | flags;
    }
}

pub fn setup_pml4(kernel_phys: u64, pml4_frame: u64) {
    let pml4 = unsafe { &mut *(pml4_frame as *mut PageTable) };

    // Map 0xffff800000000000 → kernel_phys (1 GiB huge page)
    // Index 256 in PML4 = 0xffff800000000000
    // 1 GiB page: flags = present | writable | huge
    let flags: u64 = 1 | 2 | (1 << 7);
    pml4.set_entry(256, kernel_phys, flags);

    // Identity map lower 1 GiB so we can still access code after switching page tables
    pml4.set_entry(0, 0, 1 | 2 | (1 << 7));
}
