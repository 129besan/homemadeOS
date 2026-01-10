#[repr(C)]
pub struct Elf64Header {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
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
pub struct Elf64ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
pub const ELFCLASS64: u8 = 2;
pub const ELFDATA2LSB: u8 = 1;
pub const EM_X86_64: u16 = 62;

pub const PT_LOAD: u32 = 1;
pub const PF_X: u32 = 1;
pub const PF_W: u32 = 2;
pub const PF_R: u32 = 4;

pub fn validate_elf(header: &Elf64Header) -> bool {
    if header.magic != ELF_MAGIC {
        return false;
    }
    if header.class != ELFCLASS64 {
        return false;
    }
    if header.data != ELFDATA2LSB {
        return false;
    }
    if header.e_machine != EM_X86_64 {
        return false;
    }
    true
}

pub fn program_headers<'a>(header: &'a Elf64Header) -> &'a [Elf64ProgramHeader] {
    let count = header.e_phnum as usize;
    let ptr = (header as *const Elf64Header as usize + header.e_phoff as usize) as *const Elf64ProgramHeader;
    unsafe { core::slice::from_raw_parts(ptr, count) }
}
