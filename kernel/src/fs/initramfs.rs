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

    fn header(&self) -> &InitramfsHeader {
        unsafe { &*(self.base as *const InitramfsHeader) }
    }

    fn entries(&self) -> &[InitramfsEntry] {
        let h = self.header();
        let ptr = unsafe { self.base.add(core::mem::size_of::<InitramfsHeader>()) };
        unsafe { core::slice::from_raw_parts(ptr as *const InitramfsEntry, h.file_count as usize) }
    }

    fn string_table(&self) -> &[u8] {
        let h = self.header();
        let ptr = unsafe { self.base.add(h.string_table_offset as usize) };
        let end = h.data_offset as usize;
        unsafe { core::slice::from_raw_parts(ptr, end - h.string_table_offset as usize) }
    }

    fn lookup_entry(&self, path: &str) -> Option<&InitramfsEntry> {
        let st = self.string_table();
        for entry in self.entries() {
            let name_end = st[entry.name_offset as usize..]
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(0);
            let name = core::str::from_utf8(&st[entry.name_offset as usize..][..name_end]).ok()?;
            if name == path {
                return Some(entry);
            }
        }
        None
    }

    pub fn open_file(&self, path: &str) -> Option<InitramfsFile> {
        let entry = self.lookup_entry(path)?;
        let data_ptr = unsafe { self.base.add(entry.data_offset as usize) };
        let data = unsafe { core::slice::from_raw_parts(data_ptr, entry.data_len as usize) };
        Some(InitramfsFile { data: data.to_vec(), pos: 0 })
    }
}

pub struct InitramfsFile {
    data: Vec<u8>,
    pos: usize,
}

impl FileOps for InitramfsFile {
    fn read(&self, _buf: &mut [u8]) -> Result<usize, Errno> {
        Err(Errno::ENOSYS)
    }
    fn write(&self, _buf: &[u8]) -> Result<usize, Errno> {
        Err(Errno::ENOSYS)
    }
    fn seek(&self, _offset: SeekFrom) -> Result<u64, Errno> {
        Err(Errno::ENOSYS)
    }
}

impl InitramfsFile {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Errno> {
        let remaining = self.data.len().saturating_sub(self.pos);
        let to_read = buf.len().min(remaining);
        if to_read > 0 {
            buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
            self.pos += to_read;
        }
        Ok(to_read)
    }

    pub fn seek(&mut self, offset: SeekFrom) -> Result<u64, Errno> {
        let new_pos = match offset {
            SeekFrom::Start(n) => n as usize,
            SeekFrom::Current(n) => self.pos.saturating_add(n as usize),
            SeekFrom::End(n) => self.data.len().saturating_add(n as usize),
        };
        self.pos = new_pos.min(self.data.len());
        Ok(self.pos as u64)
    }
}

use crate::fs::vfs::SeekFrom;
