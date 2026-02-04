use crate::mm::addr::VirtAddr;

pub fn flush_tlb(virt: VirtAddr) {
    unsafe {
        core::arch::asm!("invlpg ({0})", in(reg) virt.0, options(nostack, preserves_flags));
    }
}

pub fn flush_tlb_all() {
    unsafe {
        let cr3: u64;
        core::arch::asm!("mov {0}, cr3", out(reg) cr3);
        core::arch::asm!("mov cr3, {0}", in(reg) cr3);
    }
}
