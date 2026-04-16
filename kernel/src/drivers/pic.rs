const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xa0;
const PIC2_DATA: u16 = 0xa1;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;

pub fn init(offset1: u8, offset2: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") PIC1_CMD, in("al") ICW1_INIT);
        core::arch::asm!("out dx, al", in("dx") PIC2_CMD, in("al") ICW1_INIT);
        core::arch::asm!("out dx, al", in("dx") PIC1_DATA, in("al") offset1);
        core::arch::asm!("out dx, al", in("dx") PIC2_DATA, in("al") offset2);
        core::arch::asm!("out dx, al", in("dx") PIC1_DATA, in("al") 4u8);
        core::arch::asm!("out dx, al", in("dx") PIC2_DATA, in("al") 2u8);
        core::arch::asm!("out dx, al", in("dx") PIC1_DATA, in("al") ICW4_8086);
        core::arch::asm!("out dx, al", in("dx") PIC2_DATA, in("al") ICW4_8086);
        core::arch::asm!("out dx, al", in("dx") PIC1_DATA, in("al") 0xffu8);
        core::arch::asm!("out dx, al", in("dx") PIC2_DATA, in("al") 0xffu8);
    }
}

pub fn end_of_interrupt(irq: u8) {
    unsafe {
        if irq >= 8 {
            core::arch::asm!("out dx, al", in("dx") PIC2_CMD, in("al") 0x20u8);
        }
        core::arch::asm!("out dx, al", in("dx") PIC1_CMD, in("al") 0x20u8);
    }
}

pub fn unmask_irq(irq: u8) {
    let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
    let irq = if irq < 8 { irq } else { irq - 8 };
    unsafe {
        let mut mask: u8;
        core::arch::asm!("in al, dx", in("dx") port, out("al") mask);
        mask &= !(1 << irq);
        core::arch::asm!("out dx, al", in("dx") port, in("al") mask);
    }
}

pub fn disable_pic() {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") PIC1_CMD, in("al") 0xffu8);
        core::arch::asm!("out dx, al", in("dx") PIC2_CMD, in("al") 0xffu8);
        core::arch::asm!("out dx, al", in("dx") PIC1_DATA, in("al") 0xffu8);
        core::arch::asm!("out dx, al", in("dx") PIC2_DATA, in("al") 0xffu8);
    }
}
