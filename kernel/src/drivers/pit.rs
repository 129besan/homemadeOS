const PIT_CMD: u16 = 0x43;
const PIT_DATA: u16 = 0x40;

pub static TICK_COUNT: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub fn init(frequency: u32) {
    let divisor = 1193182 / frequency;
    unsafe {
        core::arch::asm!("out dx, al", in("dx") PIT_CMD, in("al") 0x36u8);
        core::arch::asm!("out dx, al", in("dx") PIT_DATA, in("al") (divisor & 0xff) as u8);
        core::arch::asm!("out dx, al", in("dx") PIT_DATA, in("al") ((divisor >> 8) & 0xff) as u8);
    }
}
