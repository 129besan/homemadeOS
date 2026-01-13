#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod elf_loader;
mod handoff;
mod memory_map;
mod uefi;

use core::ffi::c_void;
use elf_loader::{validate_elf, program_headers, Elf64Header, PT_LOAD};
use handoff::BootInfo;
use memory_map::MemoryDescriptor;
use uefi::SystemTable;

static mut BOOT_INFO: BootInfo = BootInfo {
    memory_map_ptr: 0,
    memory_map_len: 0,
    framebuffer_base: 0,
    framebuffer_width: 0,
    framebuffer_height: 0,
    framebuffer_stride: 0,
    framebuffer_format: handoff::PixelFormat::PixelBltOnly,
    kernel_phys_start: 0,
    kernel_phys_end: 0,
    initramfs_start: 0,
    initramfs_len: 0,
    rsdp_addr: 0,
};

#[panic_handler]
fn panic(_info: &core::panic::Panick_info) -> ! {
    loop {}
}

fn kernel_phys_range() -> (u64, u64) {
    (0x100000, 0x200000)
}

fn load_kernel(data: &[u8]) -> Option<u64> {
    let header = unsafe { &*(data.as_ptr() as *const Elf64Header) };
    if !validate_elf(header) {
        return None;
    }
    let phdrs = program_headers(header);
    for ph in phdrs {
        if ph.p_type == PT_LOAD {
            let start = ph.p_vaddr;
            let end = start + ph.p_memsz;
            let file_start = ph.p_offset as usize;
            let file_end = file_start + ph.p_filesz as usize;
            if file_end > data.len() {
                return None;
            }
            let src = &data[file_start..file_end];
            let dst = start as *mut u8;
            unsafe {
                core::ptr::copy_nonoverlapping(src.as_ptr(), dst, src.len());
                if ph.p_memsz > ph.p_filesz {
                    let bss = dst.add(ph.p_filesz as usize);
                    core::ptr::write_bytes(bss, 0, (ph.p_memsz - ph.p_filesz) as usize);
                }
            }
        }
    }
    Some(header.e_entry)
}

#[export_name = "efi_main"]
pub extern "efiapi" fn efi_main(
    _image_handle: *mut c_void,
    system_table: *mut SystemTable,
) -> ! {
    let st = unsafe { &*system_table };

    let hello: &[u16] = &[
        'H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16,
        ' ' as u16, 'f' as u16, 'r' as u16, 'o' as u16, 'm' as u16,
        ' ' as u16, 'M' as u16, 'y' as u16, 'O' as u16, 'S' as u16,
        '!' as u16, '\r' as u16, '\n' as u16, 0,
    ];
    uefi::print(st, hello);

    let boot_info = unsafe { &mut BOOT_INFO };
    boot_info.initramfs_start = 0;
    boot_info.initramfs_len = 0;

    let bt = st.boot_services;
    if !bt.is_null() {
        let exit: extern "efiapi" fn(*mut c_void, usize, usize) -> usize =
            unsafe { core::mem::transmute((*bt).exit_boot_services) };
        let _ = exit(_image_handle, 0, 0);
    }

    loop {}
}
