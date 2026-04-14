use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tid(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pid(pub u64);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Runnable,
    Running,
    Sleeping,
    Zombie,
}

#[repr(C)]
pub struct CpuContext {
    pub rsp: u64,
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
}

pub struct Thread {
    pub tid: Tid,
    pub pid: Pid,
    pub state: ThreadState,
    pub kernel_stack: Option<&'static mut [u8]>,
    pub context: CpuContext,
    pub entry: Option<extern "C" fn()>,
}

