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
    use crate::mm::addr::{PhysFrame, VirtAddr, PAGE_SIZE};
    use crate::mm::frame_allocator::FRAME_ALLOCATOR;
    use crate::mm::paging::flags::PageFlags;
    use crate::mm::paging::page_table::{map_page, PageTable};

    const DEFAULT_MMAP_BASE: u64 = 0x0000_4000_0000_8000;

    let len = rsi;
    if len == 0 {
        return -1;
    }

    let start = if rdi == 0 {
        DEFAULT_MMAP_BASE
    } else {
        rdi & !(PAGE_SIZE - 1)
    };
    let pages = (len + PAGE_SIZE - 1) / PAGE_SIZE;

    let cr3: u64;
    unsafe { core::arch::asm!("mov {}, cr3", out(reg) cr3); }
    let pml4 = unsafe { &mut *((cr3 & !0xfff) as *mut PageTable) };

    let mut mapped_pages = 0;
    let mut alloc_guard = FRAME_ALLOCATOR.lock();
    let allocator = match alloc_guard.as_mut() {
        Some(allocator) => allocator,
        None => return -1,
    };

    for page in 0..pages {
        let frame = match allocator.alloc() {
            Some(frame) => frame,
            None => break,
        };
        let virt = VirtAddr(start + page * PAGE_SIZE);
        let flags = PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::USER;
        if map_page(pml4, virt, frame.start_addr(), flags, allocator).is_err() {
            allocator.dealloc(frame);
            break;
        }
        mapped_pages += 1;
    }

    if mapped_pages != pages {
        for page in 0..mapped_pages {
            let virt = VirtAddr(start + page * PAGE_SIZE);
            if let Ok(phys) = crate::mm::paging::page_table::unmap_page(pml4, virt) {
                allocator.dealloc(PhysFrame::from_addr(phys));
            }
        }
        return -1;
    }

    crate::log_info!("sys_mmap: addr={:#x} len={}", start, len);
    start as isize
}

pub fn sys_munmap(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    use crate::mm::addr::{PhysFrame, VirtAddr, PAGE_SIZE};
    use crate::mm::frame_allocator::FRAME_ALLOCATOR;
    use crate::mm::paging::page_table::{unmap_page, PageTable};

    let start = rdi & !(PAGE_SIZE - 1);
    let len = rsi;
    if len == 0 {
        return -1;
    }
    let pages = (len + PAGE_SIZE - 1) / PAGE_SIZE;

    let cr3: u64;
    unsafe { core::arch::asm!("mov {}, cr3", out(reg) cr3); }
    let pml4 = unsafe { &mut *((cr3 & !0xfff) as *mut PageTable) };

    let mut alloc_guard = FRAME_ALLOCATOR.lock();
    let allocator = match alloc_guard.as_mut() {
        Some(allocator) => allocator,
        None => return -1,
    };

    for page in 0..pages {
        let virt = VirtAddr(start + page * PAGE_SIZE);
        let phys = match unmap_page(pml4, virt) {
            Ok(phys) => phys,
            Err(()) => return -1,
        };
        allocator.dealloc(PhysFrame::from_addr(phys));
    }

    crate::log_info!("sys_munmap: addr={:#x} len={}", start, len);
    0
}
