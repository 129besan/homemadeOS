#![no_std]

const SYS_EXIT: u64 = 0;
const SYS_WRITE: u64 = 1;
const SYS_READ: u64 = 2;
const SYS_OPEN: u64 = 3;
const SYS_CLOSE: u64 = 4;
const SYS_SPAWN: u64 = 5;
const SYS_WAIT: u64 = 6;
const SYS_GETPID: u64 = 7;
const SYS_YIELD: u64 = 8;

#[inline(always)]
pub unsafe fn syscall6(sysno: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") sysno => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

pub unsafe fn syscall0(sysno: u64) -> isize {
    syscall6(sysno, 0, 0, 0, 0, 0, 0)
}

pub unsafe fn syscall1(sysno: u64, arg1: u64) -> isize {
    syscall6(sysno, arg1, 0, 0, 0, 0, 0)
}

pub unsafe fn syscall3(sysno: u64, arg1: u64, arg2: u64, arg3: u64) -> isize {
    syscall6(sysno, arg1, arg2, arg3, 0, 0, 0)
}

pub fn exit(code: i32) -> ! {
    unsafe { syscall1(SYS_EXIT, code as u64); }
    loop {}
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    unsafe { syscall3(SYS_WRITE, fd as u64, buf.as_ptr() as u64, buf.len() as u64) }
}

pub fn getpid() -> isize {
    unsafe { syscall0(SYS_GETPID) }
}

pub fn yield_now() {
    unsafe { syscall0(SYS_YIELD); }
}
