#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::Panick_info) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
