#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init()
}

fn init() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
