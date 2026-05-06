use crate::proc::elf::{validate_elf, validate_program_headers, ElfHeader};
use alloc::vec::Vec;

pub fn spawn_elf(path: &str, _argv: &[&str]) -> Result<u64, ()> {
    crate::log_info!("spawn_elf: path={}", path);
    let file = crate::fs::mount::open(path).map_err(|_| ())?;
    let mut data = Vec::new();
    let mut buf = [0u8; 4096];

    loop {
        let len = file.read(&mut buf).map_err(|_| ())?;
        if len == 0 {
            break;
        }
        data.extend_from_slice(&buf[..len]);
    }

    if data.len() < core::mem::size_of::<ElfHeader>() {
        return Err(());
    }

    let header = unsafe { &*(data.as_ptr() as *const ElfHeader) };
    if !validate_elf(header) || !validate_program_headers(header, &data) {
        return Err(());
    }

    crate::log_info!("spawn_elf: entry={:#x}", header.e_entry);
    Ok(header.e_entry)
}
