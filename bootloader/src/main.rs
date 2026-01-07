#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod uefi;

use core::ffi::c_void;
use uefi::SystemTable;

#[panic_handler]
fn panic(_info: &core::panic::Panick_info) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "efiapi" fn efi_main(
    _image_handle: *mut c_void,
    system_table: *mut SystemTable,
) -> ! {
    let st = unsafe { &*system_table };

    let msg: &[u16] = &[
        'H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16,
        ' ' as u16, 'f' as u16, 'r' as u16, 'o' as u16, 'm' as u16,
        ' ' as u16, 'M' as u16, 'y' as u16, 'O' as u16, 'S' as u16,
        '!' as u16, '\r' as u16, '\n' as u16, 0,
    ];
    uefi::print(st, msg);

    loop {}
}
