use crate::arch::x86_64::idt::Idt;

pub fn init_idt(idt: &mut Idt) {
    let ks = 0x08;
    let present = 0x8e;

    idt.entries[0x00].set_handler(exc_divide_error as u64, ks, present);
    idt.entries[0x06].set_handler(exc_invalid_opcode as u64, ks, present);
    idt.entries[0x0d].set_handler(exc_gpf as u64, ks, present);
    idt.entries[0x0e].set_handler(exc_page_fault as u64, ks, present);
    idt.entries[0x08].set_handler(exc_double_fault as u64, ks, present | 0x01);

    idt.load();
}

extern "x86-interrupt" fn exc_divide_error(_frame: &mut InterruptFrame) {
    crate::kprintln!("divide error");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_invalid_opcode(_frame: &mut InterruptFrame) {
    crate::kprintln!("invalid opcode");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_gpf(_frame: &mut InterruptFrame) {
    crate::kprintln!("general protection fault");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_page_fault(frame: &mut InterruptFrame) {
    let cr2: u64;
    unsafe { core::arch::asm!("mov {0}, cr2", out(reg) cr2) };
    crate::kprintln!("page fault at {:#x}, ip={:#x}", cr2, frame.ip);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_double_fault(frame: &mut InterruptFrame) -> ! {
    crate::kprintln!("double fault at ip={:#x}", frame.ip);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

#[repr(C)]
pub struct InterruptFrame {
    pub ip: u64,
    pub cs: u64,
    pub flags: u64,
    pub sp: u64,
    pub ss: u64,
}
