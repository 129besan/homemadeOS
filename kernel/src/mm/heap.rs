use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;

pub const HEAP_START: usize = 0xffff_9000_0000_0000;
pub const HEAP_SIZE: usize = 4 * 1024 * 1024;
pub const HEAP_END: usize = HEAP_START + HEAP_SIZE;

pub struct BumpAllocator {
    heap_start: UnsafeCell<usize>,
    heap_end: UnsafeCell<usize>,
    next: UnsafeCell<usize>,
}

unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: UnsafeCell::new(0),
            heap_end: UnsafeCell::new(0),
            next: UnsafeCell::new(0),
        }
    }

    pub fn init(&self, start: usize, size: usize) {
        unsafe {
            *self.heap_start.get() = start;
            *self.heap_end.get() = start + size;
            *self.next.get() = start;
        }
    }

    pub fn grow(&mut self, additional: usize) {
        let old_end = unsafe { *self.heap_end.get() };
        unsafe {
            *self.heap_end.get() = old_end + additional;
            *self.next.get() = old_end;
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        let next = self.next.get();
        let addr = *next;
        let aligned = (addr + align - 1) & !(align - 1);
        let end = aligned + size;
        if end > unsafe { *self.heap_end.get() } {
            return core::ptr::null_mut();
        }
        *next = end;
        aligned as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
