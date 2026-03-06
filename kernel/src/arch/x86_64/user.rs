use crate::arch::x86_64::gdt::{USER_CS, USER_DS};

pub fn prepare_user_iret_frame(
    entry: u64,
    user_stack: u64,
) -> UserIretFrame {
    UserIretFrame {
        rip: entry,
        cs: USER_CS,
        rflags: 0x202,
        rsp: user_stack,
        ss: USER_DS,
    }
}

#[repr(C)]
pub struct UserIretFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}
