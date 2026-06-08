#![no_std]
#![no_main]

use libc_lite::{open, read, write, close};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let fd = open("/etc/motd\0", 0);
    if fd >= 0 {
        let mut buf = [0u8; 64];
        loop {
            let n = read(fd as usize, &mut buf);
            if n <= 0 {
                break;
            }
            write(1, &buf[..n as usize]);
        }
        let _ = close(fd as usize);
    }
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
