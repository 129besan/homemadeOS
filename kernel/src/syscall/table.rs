use crate::syscall::numbers::*;
use crate::syscall::handler::*;

pub type SyscallFn = fn(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize;

pub static SYSCALL_TABLE: &[SyscallFn] = &[
    sys_exit,
    sys_write,
    sys_read,
    sys_open,
    sys_close,
    sys_spawn,
    sys_wait,
    sys_getpid,
    sys_yield,
    sys_mmap,
    sys_munmap,
];

pub fn dispatch_syscall(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    if (rax as usize) < SYSCALL_TABLE.len() {
        SYSCALL_TABLE[rax as usize](rax, rdi, rsi, rdx, r10, r8, r9)
    } else {
        -38
    }
}
