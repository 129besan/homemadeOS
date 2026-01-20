use core::ptr::{read_volatile, write_volatile};

const COM1: u16 = 0x3f8;

const DATA: u16 = 0;
const IER: u16 = 1;
const FCR: u16 = 2;
const LCR: u16 = 3;
const LSR: u16 = 5;

const LSR_THR_EMPTY: u8 = 1 << 5;

pub fn init() {
    let lcr = base(COM1, LCR);
    unsafe {
        write_volatile(lcr, 0x80);
        write_volatile(base(COM1, DATA), 0x01);
        write_volatile(base(COM1, IER), 0x00);
        write_volatile(lcr, 0x03);
        write_volatile(base(COM1, FCR), 0x07);
        write_volatile(base(COM1, IER), 0x00);
    }
}

pub fn write_byte(byte: u8) {
    let lsr = base(COM1, LSR);
    loop {
        let status = unsafe { read_volatile(lsr) };
        if (status & LSR_THR_EMPTY) != 0 {
            break;
        }
    }
    unsafe {
        write_volatile(base(COM1, DATA), byte);
    }
}

pub fn write_string(s: &str) {
    for &b in s.as_bytes() {
        write_byte(b);
    }
}

fn base(port: u16, offset: u16) -> *mut u8 {
    (port + offset) as *mut u8
}
