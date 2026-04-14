#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub ist: u8,
    pub flags: u8,
    pub offset_mid: u16,
    pub offset_high: u32,
    pub _reserved: u32,
}

impl IdtEntry {
    pub const fn new() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            ist: 0,
            flags: 0,
            offset_mid: 0,
            offset_high: 0,
            _reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64, selector: u16, flags: u8) {
        self.offset_low = (handler & 0xffff) as u16;
        self.offset_mid = ((handler >> 16) & 0xffff) as u16;
        self.offset_high = ((handler >> 32) & 0xffffffff) as u32;
        self.selector = selector;
        self.flags = flags;
    }
}

pub struct Idt {
    pub entries: [IdtEntry; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Idt {
            entries: [IdtEntry::new(); 256],
        }
    }

    pub fn load(&self) {
        let ptr = IdtPtr {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: self as *const Idt as u64,
        };
        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &ptr, options(readonly, nostack));
        }
    }
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}
