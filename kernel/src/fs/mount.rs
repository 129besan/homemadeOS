use crate::fs::vfs::InodeRef;
use crate::fs::initramfs::Initramfs;
use crate::BootInfo;
use crate::sync::once::Once;

static ROOT_INODE: Once<InodeRef> = Once::new();
static EMBEDDED_INITRAMFS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/initramfs.img"));

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
}
