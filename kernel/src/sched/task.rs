use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tid(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pid(u64);

pub static NEXT_TID: AtomicU64 = AtomicU64::new(1);
pub static NEXT_PID: AtomicU64 = AtomicU64::new(1);

impl Tid {
    pub fn new() -> Self {
        Tid(NEXT_TID.fetch_add(1, Ordering::SeqCst))
    }
}

impl Pid {
    pub fn new() -> Self {
        Pid(NEXT_PID.fetch_add(1, Ordering::SeqCst))
    }
}
