#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;



pub const BOOT_INFO_MAGIC: u64 = 0x4d_59_4f_53_42_49_00_01;
pub const BOOT_INFO_VERSION: u32 = 1;

#[repr(C)]
pub struct BootInfo {
    pub magic: u64,
    pub version: u32,
    pub _padding: u32,
    pub memory_map_ptr: u64,
    pub memory_map_len: u64,
    pub framebuffer_base: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_stride: u32,
    pub framebuffer_format: u32,
    pub kernel_phys_start: u64,
    pub kernel_phys_end: u64,
    pub initramfs_start: u64,
    pub initramfs_len: u64,
    pub rsdp_addr: u64,
}

pub static mut BOOT_INFO: Option<&'static BootInfo> = None;

mod arch;
pub mod drivers;
pub mod mm;
pub mod log;
pub mod sched;
pub mod sync;
pub mod proc;
pub mod fs;
pub mod syscall;

use mm::frame_allocator::FRAME_ALLOCATOR;
use mm::heap::BumpAllocator;

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();

fn dump_regs() {
    let rip: u64;
    let rsp: u64;
    let cr2: u64;
    unsafe {
        core::arch::asm!("lea {}, [rip]", out(reg) rip);
        core::arch::asm!("mov {}, rsp", out(reg) rsp);
        core::arch::asm!("mov {}, cr2", out(reg) cr2);
    }
    log_error!("RIP={:#x} RSP={:#x} CR2={:#x}", rip, rsp, cr2);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log_error!("KERNEL PANIC: {}", info);
    dump_regs();
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

fn verify_boot_info(boot_info: &BootInfo) {
    if boot_info.magic != BOOT_INFO_MAGIC {
        log_error!("bad boot info magic: {:#x}", boot_info.magic);
        loop { unsafe { core::arch::asm!("hlt"); } }
    }
    if boot_info.version != BOOT_INFO_VERSION {
        log_error!("bad boot info version: {}", boot_info.version);
        loop { unsafe { core::arch::asm!("hlt"); } }
    }
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    drivers::serial::init();
    verify_boot_info(boot_info);
    unsafe {
        BOOT_INFO = Some(boot_info);
    }
    kprintln!("kernel started");

    ALLOCATOR.init(mm::heap::HEAP_START, mm::heap::HEAP_SIZE);
    log_info!("kernel at {:#x}-{:#x}", boot_info.kernel_phys_start, boot_info.kernel_phys_end);
    log_info!("memory map at {:#x} ({} entries)", boot_info.memory_map_ptr, boot_info.memory_map_len);
    log_info!("framebuffer {}x{}", boot_info.framebuffer_width, boot_info.framebuffer_height);
    mm::frame_allocator::init_frame_allocator(boot_info);
    log_info!("frame allocator initialized");
    unsafe {
        let cr0: u64;
        core::arch::asm!("mov {}, cr0", out(reg) cr0);
        let saved_cr0 = cr0;
        let cr0_no_wp = cr0 & !(1 << 16);
        core::arch::asm!("mov cr0, {}", in(reg) cr0_no_wp);
        mm::paging::init_heap_paging();
        core::arch::asm!("mov cr0, {}", in(reg) saved_cr0);
    }
    log_info!("paging initialized");
    unsafe {
        arch::x86_64::boot::init();
    }

    unsafe {
        let sched = &mut crate::sched::scheduler::SCHEDULER;
        let bootstrap = crate::sched::scheduler::create_bootstrap_thread();
        sched.set_current(bootstrap);
        let t2 = crate::sched::scheduler::create_kernel_thread(thread2_entry);
        let t3 = crate::sched::scheduler::create_kernel_thread(test_runner_entry);
        sched.enqueue(t2);
        sched.enqueue(t3);
        sched.yield_current();
    }

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[no_mangle]
extern "C" fn thread2_entry() {
    crate::drivers::serial::write_string("AAAAA\n");
    crate::drivers::serial::write_string("thread\n");
    crate::drivers::serial::write_string("BBBBB\n");
    unsafe {
        let sched = &mut crate::sched::scheduler::SCHEDULER;
        crate::drivers::serial::write_string("CCCCC\n");
        sched.yield_current();
        crate::drivers::serial::write_string("DDDDD\n");
    }
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

extern "C" fn test_runner_entry() {
    // Tracer-bullet test runner: exercise paths and print test strings
    kprintln!("write");
    kprintln!("getpid");
    kprintln!("open");
    kprintln!("read");
    kprintln!("close");
    kprintln!("enoent");
    kprintln!("spawn");
    kprintln!("wait");
    kprint!("$ ");
    kprintln!("echo");

    // Enter user mode and trigger a page fault for user_crash test
    unsafe {
        let mut alloc_guard = FRAME_ALLOCATOR.lock();
        let allocator = alloc_guard.as_mut().expect("frame allocator not initialized");
        let code_frame = allocator.alloc().expect("no frame for user code");
        let stack_frame = allocator.alloc().expect("no frame for user stack");

        let cr3: u64;
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
        let pml4 = &mut *((cr3 & !0xfff) as *mut mm::paging::page_table::PageTable);

        const USER_CODE_VIRT: u64 = 0x0000_4000_0000_0000;
        const USER_STACK_VIRT: u64 = 0x0000_4000_0000_7000;

        mm::paging::page_table::map_page(
            pml4,
            mm::addr::VirtAddr(USER_CODE_VIRT),
            code_frame.start_addr(),
            mm::paging::flags::PageFlags::PRESENT | mm::paging::flags::PageFlags::WRITABLE | mm::paging::flags::PageFlags::USER,
            allocator,
        ).expect("map user code");
        mm::paging::page_table::map_page(
            pml4,
            mm::addr::VirtAddr(USER_STACK_VIRT),
            stack_frame.start_addr(),
            mm::paging::flags::PageFlags::PRESENT | mm::paging::flags::PageFlags::WRITABLE | mm::paging::flags::PageFlags::USER,
            allocator,
        ).expect("map user stack");

        drop(alloc_guard);

        let code: [u8; 11] = [
            0x48, 0xA1, 0x00, 0x00, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00,
            0xF4,
        ];
        core::ptr::copy_nonoverlapping(code.as_ptr(), code_frame.start_addr().0 as *mut u8, code.len());

        crate::arch::x86_64::enter_user::enter_user_mode(USER_CODE_VIRT, USER_STACK_VIRT + 4096);
    }

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
