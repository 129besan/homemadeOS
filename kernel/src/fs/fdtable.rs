use crate::fs::vfs::FileRef;
use alloc::vec::Vec;

pub struct FileTable {
    files: Vec<Option<FileRef>>,
}

impl FileTable {
    pub fn new() -> Self {
        FileTable { files: Vec::new() }
    }

    pub fn insert(&mut self, file: FileRef) -> usize {
        for (i, entry) in self.files.iter_mut().enumerate() {
            if entry.is_none() {
                *entry = Some(file);
                return i;
            }
        }
        let fd = self.files.len();
        self.files.push(Some(file));
        fd
    }

    pub fn get(&self, fd: usize) -> Option<&FileRef> {
        self.files.get(fd)?.as_ref()
    }

    pub fn close(&mut self, fd: usize) {
        if let Some(entry) = self.files.get_mut(fd) {
            *entry = None;
        }
    }
}
