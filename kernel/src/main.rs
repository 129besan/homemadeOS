#![no_std]
#![no_main]

use core::ffi::c_void;

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

use mm::heap::LinkedListAllocator;

#[global_allocator]
pub static ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

#[panic_handler]
fn panic(_info: &core::panic::Panick_info) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

fn verify_boot_info(boot_info: &BootInfo) {
    if boot_info.magic != BOOT_INFO_MAGIC {
        loop { unsafe { core::arch::asm!("hlt"); } }
    }
    if boot_info.version != BOOT_INFO_VERSION {
        loop { unsafe { core::arch::asm!("hlt"); } }
    }
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    verify_boot_info(boot_info);
    unsafe {
        BOOT_INFO = Some(boot_info);
    }
    drivers::serial::init();
    kprintln!("kernel started");

    unsafe {
        ALLOCATOR.init(mm::heap::HEAP_START, mm::heap::HEAP_SIZE);
    }
    log_info!("kernel at {:#x}-{:#x}", boot_info.kernel_phys_start, boot_info.kernel_phys_end);
    log_info!("memory map at {:#x} ({} entries)", boot_info.memory_map_ptr, boot_info.memory_map_len);
    log_info!("framebuffer {}x{}", boot_info.framebuffer_width, boot_info.framebuffer_height);
    arch::boot::init();
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
