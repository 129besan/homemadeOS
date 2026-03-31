#![no_std]

pub unsafe fn write(fd: usize, buf: &[u8]) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") 1 => ret,
        in("rdi") fd,
        in("rsi") buf.as_ptr(),
        in("rdx") buf.len(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

pub unsafe fn read(fd: usize, buf: &mut [u8]) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") 2 => ret,
        in("rdi") fd,
        in("rsi") buf.as_mut_ptr(),
        in("rdx") buf.len(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

pub unsafe fn open(path: &str, flags: usize) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") 3 => ret,
        in("rdi") path.as_ptr(),
        in("rsi") flags,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

pub unsafe fn close(fd: usize) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") 4 => ret,
        in("rdi") fd,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

pub fn exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 0,
            in("rdi") code as u64,
            options(noreturn),
        );
    }
}

pub unsafe fn spawn(path: &str) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") 5 => ret,
        in("rdi") path.as_ptr(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

pub unsafe fn wait(pid: usize) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") 6 => ret,
        in("rdi") pid,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

pub fn getpid() -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") 7 => ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack),
        );
    }
    ret
}
