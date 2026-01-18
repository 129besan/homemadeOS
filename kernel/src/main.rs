#![no_std]
#![no_main]

use core::ffi::c_void;

#[repr(C)]
pub struct BootInfo {
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

#[panic_handler]
fn panic(_info: &core::panic::Panick_info) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    unsafe {
        BOOT_INFO = Some(boot_info);
    }
    arch::boot::init();
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
