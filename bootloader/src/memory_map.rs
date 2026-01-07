#[repr(C)]
pub struct MemoryDescriptor {
    pub descriptor_type: u32,
    pub padding: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

pub const EFI_MEMORY_TYPE_CONVENTIONAL: u32 = 7;
pub const EFI_MEMORY_TYPE_LOADER_DATA: u32 = 11;
pub const EFI_MEMORY_TYPE_BOOT_SERVICES_DATA: u32 = 12;
pub const EFI_MEMORY_TYPE_BOOT_SERVICES_CODE: u32 = 13;

pub fn memory_type_name(t: u32) -> &'static str {
    match t {
        1 => "EfiReserved",
        2 => "EfiLoaderCode",
        3 => "EfiLoaderData",
        4 => "EfiBootServicesCode",
        5 => "EfiBootServicesData",
        6 => "EfiRuntimeServicesCode",
        7 => "EfiConventionalMemory",
        8 => "EfiUnusableMemory",
        9 => "EfiACPIReclaim",
        10 => "EfiACPIMemoryNVS",
        11 => "EfiMemoryMappedIO",
        12 => "EfiMemoryMappedIOPort",
        13 => "EfiPalCode",
        14 => "EfiPersistentMemory",
        15 => "EfiMaxMemoryType",
        _ => "Unknown",
    }
}
