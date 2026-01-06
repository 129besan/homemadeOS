#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::ffi::c_void;

#[panic_handler]
fn panic(_info: &core::panic::Panick_info) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "efiapi" fn efi_main(
    _image_handle: *mut c_void,
    _system_table: *mut c_void,
) -> ! {
    loop {}
}
