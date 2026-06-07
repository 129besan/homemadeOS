#![no_std]
#![no_main]

use libc_lite::write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write(1, b"/bin/hello\n/bin/echo\n/bin/cat\n/bin/ls\n/bin/shell\n/etc/motd\n");
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
