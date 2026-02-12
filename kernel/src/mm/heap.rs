use core::alloc::{GlobalAlloc, Layout};

pub const HEAP_START: usize = 0xffff_9000_0000_0000;
pub const HEAP_SIZE: usize = 4 * 1024 * 1024;
pub const HEAP_END: usize = HEAP_START + HEAP_SIZE;

pub struct LinkedListAllocator {
    head: Option<&'static mut FreeBlock>,
    heap_start: usize,
    heap_end: usize,
}

struct FreeBlock {
    size: usize,
    next: Option<&'static mut FreeBlock>,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        LinkedListAllocator {
            head: None,
            heap_start: 0,
            heap_end: 0,
        }
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.heap_start = start;
        self.heap_end = start + size;
        let block = unsafe { &mut *(start as *mut FreeBlock) };
        block.size = size;
        block.next = None;
        self.head = Some(unsafe { &mut *(start as *mut FreeBlock) });
    }

    pub fn grow(&mut self, additional: usize) {
        let old_end = self.heap_end;
        self.heap_end += additional;
        let block = unsafe { &mut *(old_end as *mut FreeBlock) };
        block.size = additional;
        block.next = self.head.take();
        self.head = Some(block);
    }
}

unsafe impl GlobalAlloc for LinkedListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(16);
        let align = layout.align().max(16);
        let head = self.head as *const Option<&'static mut FreeBlock> as *mut Option<&'static mut FreeBlock>;
        let current = &mut *head;

        let mut prev: *mut Option<&'static mut FreeBlock> = current;
        let mut block = current.as_mut().and_then(|b| b.next.take());

        while let Some(mut b) = block {
            let addr = b as *const FreeBlock as usize;
            let aligned = (addr + align - 1) & !(align - 1);
            let header_size = aligned - addr;

            if b.size >= size + header_size {
                let remaining = b.size - size - header_size;
                if remaining > 16 {
                    let new_block = unsafe { &mut *((aligned + size) as *mut FreeBlock) };
                    new_block.size = remaining;
                    new_block.next = b.next.take();
                    *prev = Some(new_block);
                } else {
                    *prev = b.next.take();
                }
                *head = *prev;
                return aligned as *mut u8;
            }

            *prev = Some(b);
            prev = &mut b.next;
            block = b.next.as_mut().and_then(|b| Some(b));
        }

        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut block = unsafe { &mut *(ptr as *mut FreeBlock) };
        block.size = layout.size().max(16);

        let head = self.head as *const Option<&'static mut FreeBlock> as *mut Option<&'static mut FreeBlock>;
        let current = &mut *head;

        let block_addr = ptr as usize;
        let block_end = block_addr + block.size;

        let mut prev: *mut Option<&'static mut FreeBlock> = current;
        let mut entry = current.as_mut().and_then(|b| b.next.take());

        while let Some(ref mut b) = entry {
            let b_addr = b as *const FreeBlock as usize;
            if b_addr > block_addr {
                break;
            }
            let b_end = b_addr + b.size;
            if b_end == block_addr {
                b.size += block.size;
                *prev = Some(unsafe { &mut *(b as *mut FreeBlock) });
                return;
            }
            let saved = Some(unsafe { &mut *(b as *mut FreeBlock) });
            *prev = saved;
            prev = &mut b.next;
            entry = b.next.as_mut().and_then(|b| Some(b));
        }

        block.next = entry;
        *prev = Some(block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_dealloc() {
        let mut heap = LinkedListAllocator::new();
        let mut mem = [0u8; 4096];
        heap.init(mem.as_ptr() as usize, mem.len());

        unsafe {
            let p = heap.alloc(Layout::new::<u64>());
            assert!(!p.is_null());
            *(p as *mut u64) = 42;
            assert_eq!(*(p as *mut u64), 42);
            heap.dealloc(p, Layout::new::<u64>());
        }
    }

    #[test]
    fn test_multiple_alloc() {
        let mut heap = LinkedListAllocator::new();
        let mut mem = [0u8; 8192];
        heap.init(mem.as_ptr() as usize, mem.len());

        unsafe {
            let a = heap.alloc(Layout::new::<u64>());
            let b = heap.alloc(Layout::new::<u64>());
            let c = heap.alloc(Layout::new::<u64>());
            assert!(!a.is_null());
            assert!(!b.is_null());
            assert!(!c.is_null());
            heap.dealloc(b, Layout::new::<u64>());
            heap.dealloc(a, Layout::new::<u64>());
            heap.dealloc(c, Layout::new::<u64>());
        }
    }
}
