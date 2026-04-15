const COM1: u16 = 0x3f8;

const DATA: u16 = 0;
const IER: u16 = 1;
const FCR: u16 = 2;
const LCR: u16 = 3;
const LSR: u16 = 5;

const LSR_THR_EMPTY: u8 = 1 << 5;

pub fn init() {
    unsafe {
        outb(COM1 + LCR, 0x80);
        outb(COM1 + DATA, 0x01);
        outb(COM1 + IER, 0x00);
        outb(COM1 + LCR, 0x03);
        outb(COM1 + FCR, 0x07);
        outb(COM1 + IER, 0x00);
    }
}

pub fn write_byte(byte: u8) {
    loop {
        let status = unsafe { inb(COM1 + LSR) };
        if (status & LSR_THR_EMPTY) != 0 {
            break;
        }
    }
    unsafe {
        outb(COM1 + DATA, byte);
    }
}

pub fn write_string(s: &str) {
    for &b in s.as_bytes() {
        write_byte(b);
    }
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
        options(nomem, nostack, preserves_flags)
    );
    value
}
