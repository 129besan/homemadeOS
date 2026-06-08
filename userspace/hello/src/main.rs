#![no_std]
#![no_main]

use libc_lite::{mmap, munmap, write, yield_now};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"hello\n";
    let addr = mmap(0, 4096, 0);
    if addr >= 0 {
        let _ = munmap(addr as usize, 4096);
    }
    yield_now();
    write(1, msg);
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
