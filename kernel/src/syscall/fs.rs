use crate::fs::errno::Errno;
use crate::fs::fdtable::FileTable;
use crate::sync::spinlock::SpinLock;

static KERNEL_FD_TABLE: SpinLock<Option<FileTable>> = SpinLock::new(None);

fn with_kernel_table<F, R>(f: F) -> R
where
    F: FnOnce(&mut FileTable) -> R,
{
    let mut table = KERNEL_FD_TABLE.lock();
    if table.is_none() {
        *table = Some(FileTable::new());
    }
    f(table.as_mut().unwrap())
}

pub fn open_path(path: &str) -> Result<usize, Errno> {
    let file = crate::fs::mount::open_file(path)?;
    Ok(with_kernel_table(|table| table.insert(file)))
}

pub fn read_fd(fd: usize, buf: &mut [u8]) -> Result<usize, Errno> {
    with_kernel_table(|table| {
        let file = table.get(fd).ok_or(Errno::EBADF)?;
        file.read(buf)
    })
}

pub fn close_fd(fd: usize) -> Result<(), Errno> {
    with_kernel_table(|table| {
        if table.get(fd).is_none() {
            return Err(Errno::EBADF);
        }
        table.close(fd);
        Ok(())
    })
}
