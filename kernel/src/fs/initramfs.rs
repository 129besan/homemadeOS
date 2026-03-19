use crate::fs::vfs::{FileRef, InodeRef, DirEntry, FileOps, InodeOps};
use crate::fs::errno::Errno;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ops::Deref;

#[repr(C)]
pub struct InitramfsHeader {
    pub magic: [u8; 8],
    pub file_count: u32,
    pub string_table_offset: u32,
    pub data_offset: u32,
}

#[repr(C)]
pub struct InitramfsEntry {
    pub name_offset: u32,
    pub data_offset: u32,
    pub data_len: u32,
}

pub struct Initramfs {
    base: *const u8,
    size: usize,
}

impl Initramfs {
    pub fn new(base: u64, size: usize) -> Option<Self> {
        if size < core::mem::size_of::<InitramfsHeader>() {
            return None;
        }
        let base = base as *const u8;
        let header = unsafe { &*(base as *const InitramfsHeader) };
        if &header.magic != b"INITRAMF" {
            return None;
        }
        Some(Initramfs { base, size })
    }
}
