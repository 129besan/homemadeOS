use crate::mm::addr::{PhysAddr, PhysFrame, PAGE_SIZE};
use crate::mm::memory_map::MemoryRegion;
use crate::sync::spinlock::SpinLock;
use crate::BootInfo;

pub static FRAME_ALLOCATOR: SpinLock<Option<FrameAllocator>> = SpinLock::new(None);

pub fn init_frame_allocator(boot_info: &BootInfo) {
    let total_memory = detect_total_memory(boot_info);
    let total_frames = (total_memory / PAGE_SIZE) as usize;
    let bitmap_size = (total_frames + 63) / 64;
    let bitmap_phys = boot_info.kernel_phys_end;
    let bitmap: &'static mut [u64] = unsafe {
        core::slice::from_raw_parts_mut(bitmap_phys as *mut u64, bitmap_size)
    };
    for slot in bitmap.iter_mut() {
        *slot = 0;
    }
    let mut alloc = FrameAllocator::new(bitmap, total_frames);
    alloc.reserve_kernel(PhysAddr(boot_info.kernel_phys_start), PhysAddr(boot_info.kernel_phys_end));
    alloc.reserve_boot_info(boot_info);
    *FRAME_ALLOCATOR.lock() = Some(alloc);
}

fn detect_total_memory(boot_info: &BootInfo) -> u64 {
    use crate::mm::memory_map::parse_memory_map;
    let regions = parse_memory_map(boot_info);
    let mut max: u64 = 0;
    for r in regions {
        let end = r.start + r.length;
        if end > max {
            max = end;
        }
    }
    max
}

pub struct FrameAllocator {
    bitmap: &'static mut [u64],
    total_frames: usize,
    used_frames: usize,
}

impl FrameAllocator {
    pub fn new(bitmap: &'static mut [u64], total_frames: usize) -> Self {
        FrameAllocator {
            bitmap,
            total_frames,
            used_frames: 0,
        }
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.total_frames, self.used_frames, self.total_frames - self.used_frames)
    }

    pub fn alloc(&mut self) -> Option<PhysFrame> {
        for i in 0..self.total_frames {
            let idx = i / 64;
            let bit = i % 64;
            if idx >= self.bitmap.len() {
                break;
            }
            if (self.bitmap[idx] & (1 << bit)) == 0 {
                self.bitmap[idx] |= 1 << bit;
                self.used_frames += 1;
                return Some(PhysFrame { number: i as u64 });
            }
        }
        None
    }

    pub fn dealloc(&mut self, frame: PhysFrame) {
        let i = frame.number as usize;
        if i >= self.total_frames {
            return;
        }
        let idx = i / 64;
        let bit = i % 64;
        if idx < self.bitmap.len() {
            let old = self.bitmap[idx];
            self.bitmap[idx] = old & !(1 << bit);
            self.used_frames -= 1;
        }
    }

    pub fn reserve_region(&mut self, start: PhysAddr, length: usize) {
        let first_frame = PhysFrame::from_addr(start).number as usize;
        let num_frames = (length + PAGE_SIZE as usize - 1) / PAGE_SIZE as usize;
        for i in first_frame..(first_frame + num_frames).min(self.total_frames) {
            self.set_used(i);
        }
    }

    pub fn reserve_kernel(&mut self, phys_start: PhysAddr, phys_end: PhysAddr) {
        let len = (phys_end.0 - phys_start.0) as usize;
        self.reserve_region(phys_start, len);
    }

    pub fn reserve_boot_info(&mut self, boot_info: &BootInfo) {
        let bi_addr = PhysAddr(boot_info as *const BootInfo as u64);
        self.reserve_region(bi_addr, core::mem::size_of::<BootInfo>());

        let fb_base = PhysAddr(boot_info.framebuffer_base);
        let fb_size = (boot_info.framebuffer_stride as u64 * boot_info.framebuffer_height as u64) as usize;
        if fb_base.0 != 0 {
            self.reserve_region(fb_base, fb_size);
        }

        if boot_info.memory_map_ptr != 0 {
            let mmap_size = boot_info.memory_map_len as usize * core::mem::size_of::<MemoryRegion>();
            self.reserve_region(PhysAddr(boot_info.memory_map_ptr), mmap_size);
        }
    }

    fn set_used(&mut self, frame: usize) {
        let idx = frame / 64;
        let bit = frame % 64;
        if idx < self.bitmap.len() {
            let old = self.bitmap[idx];
            self.bitmap[idx] = old | (1 << bit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_free() {
        let mut bitmap = [0u64; 16];
        let mut alloc = FrameAllocator::new(&mut bitmap, 64);
        let frame = alloc.alloc().expect("should allocate");
        assert_eq!(frame.number, 0);
        assert_eq!(alloc.used_frames, 1);
        alloc.dealloc(frame);
        assert_eq!(alloc.used_frames, 0);
        let frame2 = alloc.alloc().expect("should allocate again");
        assert_eq!(frame2.number, 0);
    }

    #[test]
    fn test_exhaustion() {
        let mut bitmap = [0u64; 1];
        let mut alloc = FrameAllocator::new(&mut bitmap, 4);
        for i in 0..4 {
            assert!(alloc.alloc().is_some(), "frame {} should alloc", i);
        }
        assert!(alloc.alloc().is_none(), "should be exhausted");
    }

    #[test]
    fn test_reserve_region() {
        let mut bitmap = [0u64; 16];
        let mut alloc = FrameAllocator::new(&mut bitmap, 64);
        alloc.reserve_region(PhysAddr(0), 8192);
        assert!(alloc.alloc().map(|f| f.number) > Some(1));
    }
}
