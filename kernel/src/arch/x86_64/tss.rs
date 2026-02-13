#[repr(C, packed)]
pub struct TaskStateSegment {
    pub _reserved1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    pub _reserved2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub _reserved3: u64,
    pub _reserved4: u16,
    pub io_map_base: u16,
}

impl TaskStateSegment {
    pub const fn new() -> Self {
        TaskStateSegment {
            _reserved1: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            _reserved2: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            _reserved3: 0,
            _reserved4: 0,
            io_map_base: 0,
        }
    }
}
