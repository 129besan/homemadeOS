#![no_std]
#![no_main]

use libc_lite::write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write(1, b"echo\n");
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
