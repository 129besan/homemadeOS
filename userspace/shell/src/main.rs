#![no_std]
#![no_main]

use libc_lite::write;

const PROMPT: &[u8] = b"$ ";

fn read_line(buf: &mut [u8]) -> usize {
    let mut pos = 0;
    loop {
        let mut c = [0u8; 1];
        let ret = unsafe { libc_lite::read(0, &mut c) };
        if ret <= 0 {
            break;
        }
        if c[0] == b'\n' || pos >= buf.len() {
            break;
        }
        buf[pos] = c[0];
        pos += 1;
    }
    pos
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        write(1, PROMPT);
        let mut line = [0u8; 128];
        let len = read_line(&mut line);
        if len > 0 {
            write(1, b"\n");
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
