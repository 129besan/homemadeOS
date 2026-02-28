use crate::sched::task::Thread;
use alloc::collections::VecDeque;

pub struct Scheduler {
    run_queue: VecDeque<&'static mut Thread>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            run_queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, thread: &'static mut Thread) {
        self.run_queue.push_back(thread);
    }

    pub fn dequeue(&mut self) -> Option<&'static mut Thread> {
        self.run_queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.run_queue.is_empty()
    }
}
