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

fn split_argv(line: &[u8], argv: &mut [&[u8]]) -> usize {
    let mut count = 0;
    let mut start = 0;
    let mut in_token = false;
    for i in 0..line.len() {
        if line[i] == b' ' || line[i] == b'\t' {
            if in_token {
                if count < argv.len() {
                    argv[count] = &line[start..i];
                }
                count += 1;
                in_token = false;
            }
        } else if !in_token {
            start = i;
            in_token = true;
        }
    }
    if in_token && count < argv.len() {
        argv[count] = &line[start..];
        count += 1;
    }
    count
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        write(1, PROMPT);
        let mut line = [0u8; 128];
        let len = read_line(&mut line);
        if len == 0 {
            continue;
        }
        let mut args = [&[][..]; 16];
        let argc = split_argv(&line[..len], &mut args);
        if argc == 0 {
            continue;
        }
        if args[0] == b"exit" {
            break;
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
