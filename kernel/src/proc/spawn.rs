use crate::proc::elf::{validate_elf, validate_program_headers, ElfHeader};

pub fn spawn_elf(path: &str, _argv: &[&str]) -> Result<u64, ()> {
    crate::log_info!("spawn_elf: path={}", path);
    let file = crate::fs::mount::open(path).map_err(|_| ())?;
    let data = file.as_slice();

    if data.len() < core::mem::size_of::<ElfHeader>() {
        return Err(());
    }

    let header = unsafe { &*(data.as_ptr() as *const ElfHeader) };
    if !validate_elf(header) || !validate_program_headers(header, &data) {
        return Err(());
    }

    let mut address_space = crate::mm::paging::address_space::AddressSpace::new_user();
    let entry = crate::proc::elf::map_loadable_segments(data, &mut address_space)?;
    crate::log_info!("spawn_elf: entry={:#x}", entry);
    Ok(entry)
}
