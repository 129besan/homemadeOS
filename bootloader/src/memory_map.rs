#[repr(C)]
pub struct MemoryDescriptor {
    pub descriptor_type: u32,
    pub padding: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

pub const EFI_RESERVED_MEMORY_TYPE: u32 = 0;
pub const EFI_LOADER_CODE: u32 = 1;
pub const EFI_LOADER_DATA: u32 = 2;
pub const EFI_BOOT_SERVICES_CODE: u32 = 3;
pub const EFI_BOOT_SERVICES_DATA: u32 = 4;
pub const EFI_RUNTIME_SERVICES_CODE: u32 = 5;
pub const EFI_RUNTIME_SERVICES_DATA: u32 = 6;
pub const EFI_CONVENTIONAL_MEMORY: u32 = 7;
pub const EFI_UNUSABLE_MEMORY: u32 = 8;
pub const EFI_ACPI_RECLAIM_MEMORY: u32 = 9;
pub const EFI_ACPI_MEMORY_NVS: u32 = 10;
pub const EFI_MEMORY_MAPPED_IO: u32 = 11;
pub const EFI_MEMORY_MAPPED_IO_PORT_SPACE: u32 = 12;
pub const EFI_PAL_CODE: u32 = 13;
pub const EFI_PERSISTENT_MEMORY: u32 = 14;

pub const REGION_USABLE: u32 = 1;
pub const REGION_RESERVED: u32 = 2;
pub const REGION_ACPI_RECLAIMABLE: u32 = 3;
pub const REGION_ACPI_NVS: u32 = 4;
pub const REGION_BAD_MEMORY: u32 = 5;

pub fn memory_type_name(t: u32) -> &'static str {
    match t {
        EFI_RESERVED_MEMORY_TYPE => "EfiReserved",
        EFI_LOADER_CODE => "EfiLoaderCode",
        EFI_LOADER_DATA => "EfiLoaderData",
        EFI_BOOT_SERVICES_CODE => "EfiBootServicesCode",
        EFI_BOOT_SERVICES_DATA => "EfiBootServicesData",
        EFI_RUNTIME_SERVICES_CODE => "EfiRuntimeServicesCode",
        EFI_RUNTIME_SERVICES_DATA => "EfiRuntimeServicesData",
        EFI_CONVENTIONAL_MEMORY => "EfiConventionalMemory",
        EFI_UNUSABLE_MEMORY => "EfiUnusableMemory",
        EFI_ACPI_RECLAIM_MEMORY => "EfiACPIReclaim",
        EFI_ACPI_MEMORY_NVS => "EfiACPIMemoryNVS",
        EFI_MEMORY_MAPPED_IO => "EfiMemoryMappedIO",
        EFI_MEMORY_MAPPED_IO_PORT_SPACE => "EfiMemoryMappedIOPort",
        EFI_PAL_CODE => "EfiPalCode",
        EFI_PERSISTENT_MEMORY => "EfiPersistentMemory",
        _ => "Unknown",
    }
}

pub fn region_type_from_efi(t: u32) -> u32 {
    match t {
        EFI_LOADER_CODE
        | EFI_LOADER_DATA
        | EFI_BOOT_SERVICES_CODE
        | EFI_BOOT_SERVICES_DATA
        | EFI_CONVENTIONAL_MEMORY => REGION_USABLE,
        EFI_ACPI_RECLAIM_MEMORY => REGION_ACPI_RECLAIMABLE,
        EFI_ACPI_MEMORY_NVS => REGION_ACPI_NVS,
        EFI_UNUSABLE_MEMORY => REGION_BAD_MEMORY,
        _ => REGION_RESERVED,
    }
}
