use crate::mm::addr::{VirtAddr, PhysAddr};

pub fn copy_from_user(dst: &mut [u8], src: u64, len: usize) -> Result<(), ()> {
    if len > dst.len() {
        return Err(());
    }
    if !validate_user_range(src, len) {
        return Err(());
    }
    unsafe {
        core::ptr::copy_nonoverlapping(src as *const u8, dst.as_mut_ptr(), len);
    }
    Ok(())
}

pub fn copy_to_user(dst: u64, src: &[u8]) -> Result<(), ()> {
    if !validate_user_range(dst, src.len()) {
        return Err(());
    }
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dst as *mut u8, src.len());
    }
    Ok(())
}

pub fn copy_str_from_user(ptr: u64, max: usize) -> Result<alloc::string::String, ()> {
    if !validate_user_range(ptr, max) {
        return Err(());
    }
    let mut buf = alloc::vec![0u8; max];
    unsafe {
        core::ptr::copy_nonoverlapping(ptr as *const u8, buf.as_mut_ptr(), max);
    }
    let len = buf.iter().position(|&b| b == 0).unwrap_or(max);
    buf.truncate(len);
    alloc::string::String::from_utf8(buf).map_err(|_| ())
}

fn validate_user_range(addr: u64, len: usize) -> bool {
    let end = addr.saturating_add(len as u64);
    if end > 0x0000_7fff_ffff_ffff {
        return false;
    }
    addr < end
}
