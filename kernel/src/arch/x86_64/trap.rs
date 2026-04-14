pub fn is_user_frame(frame: &crate::arch::x86_64::interrupts::InterruptFrame) -> bool {
    frame.cs & 0x3 == 3
}

pub fn return_to_user(_frame: &crate::arch::x86_64::interrupts::InterruptFrame) -> ! {
    crate::kprintln!("returning from user exception not yet implemented");
    loop { unsafe { core::arch::asm!("hlt"); } }
}
