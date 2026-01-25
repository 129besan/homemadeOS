use crate::mm::addr::{PhysAddr, PhysFrame, PAGE_SIZE};

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

    fn set_used(&mut self, frame: usize) {
        let idx = frame / 64;
        let bit = frame % 64;
        if idx < self.bitmap.len() {
            let old = self.bitmap[idx];
            self.bitmap[idx] = old | (1 << bit);
        }
    }

    fn set_free(&mut self, frame: usize) {
        let idx = frame / 64;
        let bit = frame % 64;
        if idx < self.bitmap.len() {
            let old = self.bitmap[idx];
            self.bitmap[idx] = old & !(1 << bit);
        }
    }
}
