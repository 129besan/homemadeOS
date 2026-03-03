use crate::sched::task::{Thread, ThreadState, CpuContext, Tid, Pid};
use crate::sched::switch::context_switch;
use alloc::collections::VecDeque;

pub static mut SCHEDULER: Scheduler = Scheduler::new();

pub struct Scheduler {
    run_queue: VecDeque<*mut Thread>,
    current: Option<*mut Thread>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            run_queue: VecDeque::new(),
            current: None,
        }
    }

    pub fn enqueue(&mut self, thread: &'static mut Thread) {
        self.run_queue.push_back(thread as *mut Thread);
    }

    pub fn dequeue(&mut self) -> Option<&'static mut Thread> {
        let ptr = self.run_queue.pop_front()?;
        Some(unsafe { &mut *ptr })
    }

    pub fn is_empty(&self) -> bool {
        self.run_queue.is_empty()
    }

    pub fn current(&self) -> Option<&'static mut Thread> {
        let ptr = self.current?;
        Some(unsafe { &mut *ptr })
    }

    pub fn yield_current(&mut self) {
        if let Some(current) = self.current.take() {
            let thread = unsafe { &mut *current };
            thread.state = ThreadState::Runnable;
            self.run_queue.push_back(current);
        }
        if let Some(next) = self.dequeue() {
            next.state = ThreadState::Running;
            self.current = Some(next as *mut Thread);
            if let Some(old) = self.current() {
                unsafe { context_switch(&mut old.context, &next.context); }
            }
        }
    }

    pub fn run_idle(&mut self) -> ! {
        self.idle()
    }

    pub fn idle() -> ! {
        loop {
            unsafe { core::arch::asm!("hlt"); }
        }
    }
}

pub fn timer_tick() {
    let sched = unsafe { &mut SCHEDULER };
    if sched.current.is_some() {
        sched.yield_current();
    }
}

pub fn create_bootstrap_thread() -> &'static mut Thread {
    let thread = alloc::boxed::Box::new(Thread {
        tid: Tid::new(),
        pid: Pid::new(),
        state: ThreadState::Running,
        kernel_stack: None,
        context: CpuContext {
            rsp: 0, r15: 0, r14: 0, r13: 0, r12: 0, rbx: 0, rbp: 0,
        },
        entry: None,
    });
    Box::leak(thread)
}
