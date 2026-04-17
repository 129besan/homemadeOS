#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod elf_loader;
mod handoff;
mod memory_map;
mod uefi;

use core::ffi::c_void;
use elf_loader::{program_header, read_header, validate_elf, PT_LOAD};
use handoff::BootInfo;
use uefi::SystemTable;

static mut BOOT_INFO: BootInfo = BootInfo {
    magic: handoff::BOOT_INFO_MAGIC,
    version: handoff::BOOT_INFO_VERSION,
    _padding: 0,
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

static mut MEMORY_MAP_BUFFER: [u8; 65536] = [0; 65536];
static mut MEMORY_REGIONS: [handoff::MemoryRegion; 256] = [handoff::MemoryRegion {
    start: 0,
    len: 0,
    region_type: 0,
}; 256];

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn load_kernel(data: &[u8]) -> Option<u64> {
    let header = read_header(data)?;
    if !validate_elf(&header) {
        return None;
    }
    for i in 0..header.e_phnum as usize {
        let ph = program_header(data, &header, i)?;
        if ph.p_type == PT_LOAD {
            let start = ph.p_vaddr;
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

    let kernel_data: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../target/x86_64-unknown-none/debug/kernel"
    ));

    let header = match read_header(kernel_data) {
        Some(header) => header,
        None => loop {},
    };
    if !validate_elf(&header) {
        loop {}
    }

    let entry = match load_kernel(kernel_data) {
        Some(e) => e,
        None => loop {},
    };

    // Find kernel physical range from PT_LOAD segments
    let mut phys_start = u64::MAX;
    let mut phys_end = 0u64;
    for i in 0..header.e_phnum as usize {
        let ph = match program_header(kernel_data, &header, i) {
            Some(ph) => ph,
            None => loop {},
        };
        if ph.p_type == PT_LOAD {
            let start = ph.p_vaddr;
            let end = start + ph.p_memsz;
            if start < phys_start { phys_start = start; }
            if end > phys_end { phys_end = end; }
        }
    }

    let boot_info = unsafe { &mut BOOT_INFO };
    boot_info.kernel_phys_start = phys_start;
    boot_info.kernel_phys_end = phys_end;
    boot_info.initramfs_start = 0;
    boot_info.initramfs_len = 0;

    let bs = unsafe { &*st.boot_services };
    let get_memory_map: uefi::GetMemoryMapFn = unsafe { core::mem::transmute(bs.get_memory_map) };
    let exit_boot_services: uefi::ExitBootServicesFn =
        unsafe { core::mem::transmute(bs.exit_boot_services) };

    let mut memory_map_size = core::mem::size_of::<[u8; 65536]>();
    let mut map_key: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;

    let status = get_memory_map(
        &mut memory_map_size,
        unsafe { MEMORY_MAP_BUFFER.as_mut_ptr() },
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );
    if status != 0 {
        loop {}
    }

    let num_entries = memory_map_size / descriptor_size;
    let region_count = convert_memory_map(
        unsafe { MEMORY_MAP_BUFFER.as_ptr() },
        num_entries,
        descriptor_size,
        unsafe { &mut MEMORY_REGIONS },
    );

    let status = exit_boot_services(_image_handle, map_key);
    if status != 0 {
        loop {}
    }

    boot_info.memory_map_ptr = unsafe { MEMORY_REGIONS.as_ptr() as u64 };
    boot_info.memory_map_len = region_count as u64;

    let entry_fn: extern "sysv64" fn(*const BootInfo) -> ! =
        unsafe { core::mem::transmute(entry as usize) };
    unsafe {
        entry_fn(&BOOT_INFO as *const BootInfo);
    }
}

fn convert_memory_map(
    efi_map: *const u8,
    efi_entries: usize,
    efi_desc_size: usize,
    out: &mut [handoff::MemoryRegion],
) -> usize {
    let mut count = 0;
    for i in 0..efi_entries {
        let desc = unsafe { efi_map.add(i * efi_desc_size) as *const memory_map::MemoryDescriptor };
        let ty = unsafe { (*desc).descriptor_type };
        let region_type = match ty {
            7 => 1,     // EfiConventionalMemory -> Usable
            4 | 5 => 1, // EfiBootServicesCode/Data -> Usable after ExitBootServices
            _ => 2,     // Reserved
        };
        let pages = unsafe { (*desc).number_of_pages };
        if pages > 0 && count < out.len() {
            out[count] = handoff::MemoryRegion {
                start: unsafe { (*desc).physical_start },
                len: pages * 4096,
                region_type,
            };
            count += 1;
        }
    }
    count
}
