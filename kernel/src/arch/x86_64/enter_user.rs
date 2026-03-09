use crate::arch::x86_64::user::UserIretFrame;
use crate::arch::x86_64::gdt::{USER_CS, USER_DS};
use crate::mm::paging::address_space::AddressSpace;

pub fn enter_user_mode(entry: u64, rsp: u64) -> ! {
    let frame = UserIretFrame {
        rip: entry,
        cs: USER_CS,
        rflags: 0x202,
        rsp,
        ss: USER_DS,
    };
    unsafe {
        core::arch::asm!(
            "push {ss}",
            "push {rsp}",
            "push {rflags}",
            "push {cs}",
            "push {rip}",
            "iretq",
            ss = in(reg) frame.ss,
            rsp = in(reg) frame.rsp,
            rflags = in(reg) frame.rflags,
            cs = in(reg) frame.cs,
            rip = in(reg) frame.rip,
        );
    }
    loop { core::arch::asm!("hlt"); }
}
