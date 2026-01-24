#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysFrame {
    pub number: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub number: u64,
}

pub const PAGE_SIZE: u64 = 4096;

impl PhysAddr {
    pub fn page_align_down(&self) -> Self {
        PhysAddr(self.0 & !(PAGE_SIZE - 1))
    }
    pub fn page_align_up(&self) -> Self {
        PhysAddr((self.0 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1))
    }
}

impl VirtAddr {
    pub fn page_align_down(&self) -> Self {
        VirtAddr(self.0 & !(PAGE_SIZE - 1))
    }
    pub fn page_align_up(&self) -> Self {
        VirtAddr((self.0 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1))
    }
}

impl PhysFrame {
    pub fn from_addr(addr: PhysAddr) -> Self {
        PhysFrame { number: addr.0 / PAGE_SIZE }
    }
    pub fn start_addr(&self) -> PhysAddr {
        PhysAddr(self.number * PAGE_SIZE)
    }
}
