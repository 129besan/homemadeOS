const KEYBOARD_DATA: u16 = 0x60;
const KEYBOARD_STATUS: u16 = 0x64;

pub fn init() {
}

pub fn read_scancode() -> u8 {
    let code: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") code, in("dx") KEYBOARD_DATA);
    }
    code
}

pub fn handle_keyboard() {
    let scancode = read_scancode();
    crate::kprintln!("key: {:02x}", scancode);
}
