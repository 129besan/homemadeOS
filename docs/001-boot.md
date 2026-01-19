# Boot Handoff Contract

## BootInfo ABI

The bootloader passes a `BootInfo` structure to the kernel entry point via the
standard calling convention (rdi = BootInfo pointer on x86_64).

```rust
pub const BOOT_INFO_MAGIC: u64 = 0x4d_59_4f_53_42_49_00_01;
pub const BOOT_INFO_VERSION: u32 = 1;

pub struct BootInfo {
    pub magic: u64,           // must equal BOOT_INFO_MAGIC
    pub version: u32,         // must equal BOOT_INFO_VERSION
    pub _padding: u32,
    pub memory_map_ptr: u64,  // physical address of memory map
    pub memory_map_len: u64,  // number of entries
    pub framebuffer_base: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_stride: u32,
    pub framebuffer_format: u32,
    pub kernel_phys_start: u64,
    pub kernel_phys_end: u64,
    pub initramfs_start: u64,
    pub initramfs_len: u64,
    pub rsdp_addr: u64,
}
```

## Ownership

- `BootInfo` lives at a fixed physical address allocated by the bootloader.
- The kernel must copy any data it needs before modifying page tables.
- Memory map entries and framebuffer are valid at the recorded addresses.

## UEFI Caveats

- After `ExitBootServices`, UEFI runtime services are not available unless
  the kernel manually maps the runtime region.
- The bootloader must not rely on pool allocations after handoff.
- Timer and interrupt state is undefined after exit.
