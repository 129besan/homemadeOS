pub fn sys_exit(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let code = rdi as i32;
    crate::log_info!("process exit with code {}", code);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

pub fn sys_write(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}

pub fn sys_read(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}

pub fn sys_open(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}

pub fn sys_close(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}

pub fn sys_spawn(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}

pub fn sys_wait(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}

pub fn sys_getpid(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    crate::log_info!("sys_getpid called");
    1
}

pub fn sys_yield(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    crate::log_info!("sys_yield called");
    crate::sched::scheduler::timer_tick();
    0
}

pub fn sys_mmap(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}

pub fn sys_munmap(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    -38
}
