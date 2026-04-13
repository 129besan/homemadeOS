# MyOS v1.0 — Minimal OS

A small x86_64 operating system that boots via UEFI, runs a higher-half
monolithic kernel, supports user-mode ELF programs, and provides a tiny shell.

## Features

- UEFI bootloader with framebuffer and memory map
- Higher-half kernel (Rust, no_std)
- Physical frame allocator (bitmap)
- 4-level page tables with direct physical map
- Linked-list heap allocator
- GDT, TSS, IDT with exception handlers
- PIT timer + PS/2 keyboard driver
- Spinlock synchronisation
- Round-robin kernel thread scheduler
- Ring 3 user mode entry via iretq
- syscall/sysret ABI (11 syscalls)
- Initramfs read-only filesystem
- ELF64 user program loader
- User programs: init, hello, echo, cat, ls, shell
- CI smoke testing infrastructure

## Limitations

- No SMP
- No ACPI
- No USB
- No networking
- No fork/COW
- No dynamic linking
- QEMU-only testing
