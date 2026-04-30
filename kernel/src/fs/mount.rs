use crate::fs::errno::Errno;
use crate::fs::initramfs::Initramfs;
use crate::sync::spinlock::SpinLock;
use crate::BootInfo;

static EMBEDDED_INITRAMFS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/initramfs.img"));
static ROOT_INITRAMFS: SpinLock<Option<Initramfs>> = SpinLock::new(None);

pub fn mount_root(boot_info: &BootInfo) {
    let (start, len) = if boot_info.initramfs_len > 0 {
        (boot_info.initramfs_start, boot_info.initramfs_len as usize)
    } else {
        (EMBEDDED_INITRAMFS.as_ptr() as u64, EMBEDDED_INITRAMFS.len())
    };

    if len == 0 {
        crate::log_warn!("no initramfs provided");
        return;
    }
    let initramfs = Initramfs::new(start, len).expect("invalid initramfs");
    crate::log_info!("initramfs mounted with {} files", initramfs.header().file_count);
    *ROOT_INITRAMFS.lock() = Some(initramfs);
}

pub fn open(path: &str) -> Result<crate::fs::initramfs::InitramfsFile, Errno> {
    let initramfs = ROOT_INITRAMFS.lock();
    let initramfs = initramfs.as_ref().ok_or(Errno::ENOENT)?;
    initramfs.lookup_and_open(path).ok_or(Errno::ENOENT)
}
