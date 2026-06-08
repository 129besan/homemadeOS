use crate::arch::x86_64::gdt::{Gdt, KERNEL_CS, KERNEL_DS};
use crate::arch::x86_64::idt::Idt;
use crate::arch::x86_64::interrupts::init_idt;
use crate::arch::x86_64::segments::{load_gdt, load_tss};
use crate::arch::x86_64::tss::TaskStateSegment;
use crate::drivers;

static mut GDT: Gdt = Gdt::new();
static mut IDT: Idt = Idt::new();
static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut TSS_STACK: [u8; 4096] = [0u8; 4096];

pub unsafe fn init() {
    crate::drivers::serial::write_string("boot init\n");

    core::arch::asm!("cli", options(nomem, nostack, preserves_flags));

    TSS.rsp0 = TSS_STACK.as_ptr().add(4096) as u64;
    GDT.set_tss(&TSS);

    load_gdt(&raw mut GDT);

    core::arch::asm!(
        "push {cs}",
        "lea {rip}, 2f",
        "push {rip}",
        "retfq",
        "2:",
        cs = in(reg) KERNEL_CS,
        rip = lateout(reg) _,
        options(preserves_flags)
    );

    let ds = KERNEL_DS as u16;
    core::arch::asm!("mov ds, ax", in("ax") ds, options(nomem, nostack));
    core::arch::asm!("mov es, ax", in("ax") ds, options(nomem, nostack));
    core::arch::asm!("mov ss, ax", in("ax") ds, options(nomem, nostack));

    load_tss();

    init_idt(&raw mut IDT);
    drivers::pic::init(0x20, 0x28);
    drivers::pit::init(100);
    drivers::pic::unmask_irq(0);

    crate::drivers::serial::write_string("before int\n");
    core::arch::asm!("int 0x20", options(nomem, nostack));
    crate::drivers::serial::write_string("after int\n");
    core::arch::asm!("int3", options(nomem, nostack));

    core::arch::asm!("sti", options(nomem, nostack));
}