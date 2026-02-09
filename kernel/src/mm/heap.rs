use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

pub const HEAP_START: usize = 0xffff_9000_0000_0000;
pub const HEAP_SIZE: usize = 4 * 1024 * 1024;
pub const HEAP_END: usize = HEAP_START + HEAP_SIZE;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: AtomicUsize::new(0),
        }
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.heap_start = start;
        self.heap_end = start + size;
        self.next.store(start, Ordering::SeqCst);
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        loop {
            let current = self.next.load(Ordering::SeqCst);
            let next = (current + align - 1) & !(align - 1);
            let end = next + size;
            if end > self.heap_end {
                return core::ptr::null_mut();
            }
            if self.next.compare_exchange_weak(current, end, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                return next as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}
