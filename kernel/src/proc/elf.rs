#[repr(C)]
pub struct ElfHeader {
    pub magic: [u8; 4],
    pub class: u8,
    pub endian: u8,
    pub version: u8,
    pub osabi: u8,
    pub abiversion: u8,
    pub padding: [u8; 7],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub const PT_LOAD: u32 = 1;
pub const ELF_MAGIC: [u8; 4] = *b"\x7fELF";

pub fn validate_elf(header: &ElfHeader) -> bool {
    if header.magic != ELF_MAGIC {
        return false;
    }
    if header.class != 2 {
        return false;
    }
    if header.endian != 1 {
        return false;
    }
    if header.e_machine != 0x3e {
        return false;
    }
    true
}

pub fn validate_program_headers(header: &ElfHeader, data: &[u8]) -> bool {
    if header.e_phnum == 0 {
        return false;
    }
    let phoff = header.e_phoff as usize;
    let phentsize = header.e_phentsize as usize;
    for i in 0..header.e_phnum as usize {
        let offset = phoff + i * phentsize;
        if offset + core::mem::size_of::<ProgramHeader>() > data.len() {
            return false;
        }
        let ph = unsafe { &*(data[offset..].as_ptr() as *const ProgramHeader) };
        if ph.p_type == PT_LOAD {
            if ph.p_align & (ph.p_align - 1) != 0 {
                return false;
            }
        }
    }
    true
}

pub fn map_loadable_segments(
    data: &[u8],
    address_space: &mut AddressSpace,
) -> Result<u64, ()> {
    let header = unsafe { &*(data.as_ptr() as *const ElfHeader) };
    let phoff = header.e_phoff as usize;
    let phentsize = header.e_phentsize as usize;
    let entry = header.e_entry;

    for i in 0..header.e_phnum as usize {
        let offset = phoff + i * phentsize;
        let ph = unsafe { &*(data[offset..].as_ptr() as *const ProgramHeader) };
        if ph.p_type != PT_LOAD {
            continue;
        }

        let file_start = ph.p_offset as usize;
        let file_end = file_start + ph.p_filesz as usize;
        let file_data = &data[file_start..file_end];
        let vaddr = ph.p_vaddr;

        crate::log_info!(
            "load segment vaddr={:#x} filesz={} memsz={} flags={}",
            vaddr, ph.p_filesz, ph.p_memsz, ph.p_flags
        );
    }

    Ok(entry)
}
