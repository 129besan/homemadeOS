const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xa0;
const PIC2_DATA: u16 = 0xa1;

pub fn disable_pic() {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") PIC1_CMD, in("al") 0xffu8);
        core::arch::asm!("out dx, al", in("dx") PIC2_CMD, in("al") 0xffu8);
        core::arch::asm!("out dx, al", in("dx") PIC1_DATA, in("al") 0xffu8);
        core::arch::asm!("out dx, al", in("dx") PIC2_DATA, in("al") 0xffu8);
    }
}
