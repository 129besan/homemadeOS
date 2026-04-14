use core::alloc::GlobalAlloc;

pub const KERNEL_STACK_SIZE: usize = 4096 * 16;

pub fn alloc_kernel_stack() -> Option<&'static mut [u8]> {
    let layout = core::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 16).ok()?;
    let ptr = unsafe { crate::ALLOCATOR.alloc(layout) };
    if ptr.is_null() {
        return None;
    }
    unsafe { Some(core::slice::from_raw_parts_mut(ptr, KERNEL_STACK_SIZE)) }
}
