use crate::fs::vfs::{FileOps, SeekFrom};
use crate::fs::errno::Errno;

pub struct Console;

impl FileOps for Console {
    fn read(&self, _buf: &mut [u8]) -> Result<usize, Errno> {
        Err(Errno::ENOSYS)
    }
    fn write(&self, buf: &[u8]) -> Result<usize, Errno> {
        if let Ok(s) = core::str::from_utf8(buf) {
            crate::kprint!("{}", s);
        }
        Ok(buf.len())
    }
    fn seek(&self, _offset: SeekFrom) -> Result<u64, Errno> {
        Err(Errno::ENOSYS)
    }
}
