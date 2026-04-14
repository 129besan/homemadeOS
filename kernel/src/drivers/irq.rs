use crate::arch::x86_64::idt::Idt;

const PIC_REMAP_OFFSET: u8 = 0x20;

type IrqHandler = fn();

static mut IRQ_HANDLERS: [Option<IrqHandler>; 16] = [None; 16];

pub fn register_irq(irq: u8, handler: IrqHandler) {
    if irq < 16 {
        unsafe {
            IRQ_HANDLERS[irq as usize] = Some(handler);
        }
    }
}

extern "x86-interrupt" fn irq_handler(frame: &mut crate::arch::x86_64::interrupts::InterruptFrame) {
    let irq = (frame.cs >> 8) & 0xff;
    if irq < 16 {
        if let Some(handler) = unsafe { IRQ_HANDLERS[irq as usize] } {
            handler();
        }
    }
}

pub fn init_irqs(idt: &mut Idt) {
    let ks = 0x08;
    let present = 0x8e;
    for i in 0..16 {
        idt.entries[(PIC_REMAP_OFFSET as usize) + i].set_handler(
            irq_handler as *const () as u64, ks, present,
        );
    }
}
