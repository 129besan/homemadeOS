# Architecture Decision Records

## ADR-001: UEFI Boot

**Status**: Accepted

**Context**: Need a boot method for x86_64.
BIOS requires real mode setup, disk I/O via int 13h, and A20 gate.

**Decision**: Use UEFI. The firmware provides:
- File system access (FAT)
- Memory map
- Framebuffer (GOP)
- ExitBootServices

**Consequence**: Higher complexity in bootloader tooling (FAT image).
Simpler kernel entry point.

## ADR-002: Monolithic Kernel

**Status**: Accepted

**Context**: Need a kernel architecture for the target OS.

**Decision**: Monolithic kernel with in-kernel drivers.
No microkernel IPC overhead.
All drivers in kernel space.

**Consequence**: Easier development. No driver crashes the kernel.

## ADR-003: Initramfs

**Status**: Accepted

**Context**: Need a root filesystem without disk drivers.

**Decision**: Embed an initramfs archive in the kernel image.
Custom simple archive format.

**Consequence**: No disk driver needed at boot.
Static filesystem that cannot be modified at runtime.
