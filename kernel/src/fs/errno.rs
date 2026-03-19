#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Errno {
    ENOENT,
    EINVAL,
    EBADF,
    EFAULT,
    ENOMEM,
    ENOSYS,
}

impl Errno {
    pub fn to_isize(&self) -> isize {
        match self {
            Errno::ENOENT => -2,
            Errno::EINVAL => -22,
            Errno::EBADF => -9,
            Errno::EFAULT => -14,
            Errno::ENOMEM => -12,
            Errno::ENOSYS => -38,
        }
    }
}
