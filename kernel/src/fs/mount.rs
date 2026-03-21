use crate::fs::vfs::InodeRef;
use crate::fs::initramfs::Initramfs;
use crate::BootInfo;
use crate::sync::once::Once;

static ROOT_INODE: Once<InodeRef> = Once::new();

pub fn mount_root(boot_info: &BootInfo) {
    if boot_info.initramfs_len == 0 {
        crate::log_warn!("no initramfs provided");
        return;
    }
    let initramfs = Initramfs::new(boot_info.initramfs_start, boot_info.initramfs_len as usize)
        .expect("invalid initramfs");
    crate::log_info!("initramfs mounted with {} files", initramfs.header().file_count);
}
