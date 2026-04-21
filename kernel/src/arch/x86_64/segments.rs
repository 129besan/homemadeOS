use super::gdt::Gdt;
use super::tss::TaskStateSegment;

pub unsafe fn load_gdt(gdt: *mut Gdt) {
    let gdt_ptr = GdtPtr {
        limit: (core::mem::size_of::<Gdt>() - 1) as u16,
        base: gdt as u64,
    };
    unsafe {
        core::arch::asm!("lgdt [{}]", in(reg) &gdt_ptr, options(readonly, nostack));
    }
}

pub fn load_tss() {
    unsafe {
        core::arch::asm!("ltr ax", in("ax") 0x28u16, options(nostack));
    }
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u64,
}
