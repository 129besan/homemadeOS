#![no_std]
#![no_main]

use libc_lite::write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"hello\n";
    write(1, msg);
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
