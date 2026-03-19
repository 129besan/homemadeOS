use crate::fs::errno::Errno;
use alloc::sync::Arc;
use core::ops::Deref;

pub type FileRef = Arc<dyn FileOps>;

pub trait FileOps: Send + Sync {
    fn read(&self, buf: &mut [u8]) -> Result<usize, Errno>;
    fn write(&self, buf: &[u8]) -> Result<usize, Errno>;
    fn seek(&self, offset: SeekFrom) -> Result<u64, Errno>;
}

pub enum SeekFrom {
    Start(u64),
    Current(i64),
    End(i64),
}

pub type InodeRef = Arc<dyn InodeOps>;

pub trait InodeOps: Send + Sync {
    fn open(&self) -> Result<FileRef, Errno>;
}

pub struct DirEntry {
    pub name: alloc::string::String,
    pub inode: InodeRef,
}
