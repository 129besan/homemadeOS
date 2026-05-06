pub fn sys_exit(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let code = rdi as i32;
    crate::log_info!("process exit with code {}", code);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

pub fn sys_write(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let fd = rdi as usize;
    let len = rdx as usize;
    let mut buf = alloc::vec![0u8; len];
    if crate::syscall::validate::copy_from_user(&mut buf, rsi, len).is_err() {
        return -14;
    }
    match fd {
        1 => {
            if let Ok(s) = core::str::from_utf8(&buf) {
                crate::kprint!("{}", s);
            }
            len as isize
        }
        _ => -9,
    }
}

pub fn sys_read(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let fd = rdi as usize;
    let buf_ptr = rsi;
    let len = rdx as usize;
    if len == 0 {
        return 0;
    }
    let mut buf = alloc::vec![0u8; len];
    crate::log_info!("sys_read: fd={} len={}", fd, len);
    let read_len = match crate::syscall::fs::read_fd(fd, &mut buf) {
        Ok(n) => n,
        Err(errno) => return errno.to_isize(),
    };
    if crate::syscall::validate::copy_to_user(buf_ptr, &buf[..read_len]).is_err() {
        return -14;
    }
    read_len as isize
}

pub fn sys_open(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let path_ptr = rdi;
    let _flags = rsi;
    let max_len = 256;
    let path = match crate::syscall::validate::copy_str_from_user(path_ptr, max_len) {
        Ok(p) => p,
        Err(_) => return -14,
    };
    crate::log_info!("sys_open: {}", path);
    match crate::syscall::fs::open_path(&path) {
        Ok(fd) => fd as isize,
        Err(errno) => errno.to_isize(),
    }
}

pub fn sys_close(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let fd = rdi as usize;
    crate::log_info!("sys_close: fd={}", fd);
    match crate::syscall::fs::close_fd(fd) {
        Ok(()) => 0,
        Err(errno) => errno.to_isize(),
    }
}

pub fn sys_spawn(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let path_ptr = rdi;
    let _argv_ptr = rsi;
    let max_len = 256;
    let path = match crate::syscall::validate::copy_str_from_user(path_ptr, max_len) {
        Ok(p) => p,
        Err(_) => return -14,
    };
    crate::log_info!("sys_spawn: {}", path);
    match crate::proc::spawn::spawn_elf(&path, &[]) {
        Ok(entry) => entry as isize,
        Err(()) => -2,
    }
}

pub fn sys_wait(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let _pid = rdi;
    crate::log_info!("sys_wait called");
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
