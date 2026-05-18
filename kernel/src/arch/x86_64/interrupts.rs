use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::tss::TaskStateSegment;

pub static DOUBLE_FAULT_STACK: [u8; 4096] = [0u8; 4096];

pub fn init_double_fault_ist(tss: &mut TaskStateSegment) {
    let stack_top = unsafe { DOUBLE_FAULT_STACK.as_ptr().add(4096) } as u64;
    tss.ist1 = stack_top;
}

pub unsafe fn init_idt(idt: *mut Idt) {
    let ks = 0x08;
    let present = 0x8e;
    let idt = &mut *idt;

    idt.entries[0x00].set_handler(exc_divide_error as *const () as u64, ks, present);
    idt.entries[0x03].set_handler(exc_breakpoint as *const () as u64, ks, present);
    idt.entries[0x06].set_handler(exc_invalid_opcode as *const () as u64, ks, present);
    idt.entries[0x0d].set_handler(exc_gpf as *const () as u64, ks, present);
    idt.entries[0x0e].set_handler(exc_page_fault as *const () as u64, ks, present);
    idt.entries[0x08].set_handler(exc_double_fault as *const () as u64, ks, present | 0x01);

    idt.entries[0x20].set_handler(irq_timer as *const () as u64, ks, present);

    idt.load();
}

extern "x86-interrupt" fn irq_timer(_frame: &mut InterruptFrame) {
    crate::drivers::pit::TICK_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    crate::drivers::serial::write_string("tick\n");
    crate::drivers::pic::end_of_interrupt(0);
}

extern "x86-interrupt" fn exc_divide_error(_frame: &mut InterruptFrame) {
    crate::kprintln!("divide error");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_breakpoint(_frame: &mut InterruptFrame) {
    crate::kprintln!("breakpoint");
}

extern "x86-interrupt" fn exc_invalid_opcode(_frame: &mut InterruptFrame) {
    crate::kprintln!("invalid opcode");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_gpf(_frame: &mut InterruptFrame, error_code: u64) {
    crate::kprintln!("general protection fault error_code={}", error_code);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_page_fault(frame: &mut InterruptFrame, error_code: u64) {
    let cr2: u64;
    unsafe { core::arch::asm!("mov {0}, cr2", out(reg) cr2) };
    let present = error_code & 1 != 0;
    let write = error_code & 2 != 0;
    let user = error_code & 4 != 0;
    crate::kprintln!(
        "page fault at {:#x}, ip={:#x}, present={}, write={}, user={}",
        cr2, frame.ip, present, write, user,
    );
    if user {
        let pid = unsafe {
            crate::sched::scheduler::SCHEDULER
                .current()
                .map(|thread| thread.pid)
                .unwrap_or(crate::sched::task::Pid(0))
        };
        crate::proc::process::kill_user_task(pid);
        crate::kprintln!("killed");
    }
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "x86-interrupt" fn exc_double_fault(frame: &mut InterruptFrame, _error_code: u64) -> ! {
    crate::kprintln!("double fault at ip={:#x}, sp={:#x}", frame.ip, frame.sp);
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
