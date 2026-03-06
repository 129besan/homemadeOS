#[repr(C, packed)]
pub struct Gdt {
    pub null: u64,
    pub kernel_code: u64,
    pub kernel_data: u64,
    pub user_code: u64,
    pub user_data: u64,
    pub tss_low: u64,
    pub tss_high: u64,
}

impl Gdt {
    pub fn new() -> Self {
        Gdt {
            null: 0,
            kernel_code: Gdt::make_desc(0, 0, GdtAccess::KERNEL_CODE),
            kernel_data: Gdt::make_desc(0, 0, GdtAccess::KERNEL_DATA),
            user_code: Gdt::make_desc(0, 0, GdtAccess::USER_CODE),
            user_data: Gdt::make_desc(0, 0, GdtAccess::USER_DATA),
            tss_low: 0,
            tss_high: 0,
        }
    }

    fn make_desc(base: u32, limit: u32, access: GdtAccess) -> u64 {
        let mut desc: u64 = 0;
        desc |= (limit as u64 & 0xffff) << 0;
        desc |= (base as u64 & 0xffffff) << 16;
        desc |= (access.bits() as u64) << 40;
        desc |= ((limit >> 16) as u64 & 0xf) << 48;
        desc |= (1u64 << 55);
        desc |= ((base >> 24) as u64 & 0xff) << 56;
        desc
    }
}

pub struct GdtAccess(u8);

impl GdtAccess {
    pub const KERNEL_CODE: GdtAccess = GdtAccess(0x9a);
    pub const KERNEL_DATA: GdtAccess = GdtAccess(0x92);
    pub const USER_CODE: GdtAccess = GdtAccess(0xfa);
    pub const USER_DATA: GdtAccess = GdtAccess(0xf2);

    pub fn bits(&self) -> u64 {
        self.0 as u64
    }
}

pub const KERNEL_CS: u64 = 0x08;
pub const KERNEL_DS: u64 = 0x10;
pub const USER_CS: u64 = 0x18 | 3;
pub const USER_DS: u64 = 0x20 | 3;
