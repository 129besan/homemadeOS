# MyOS Overview

## Goal

A small x86_64 operating system that boots via UEFI, runs a higher-half
monolithic kernel, supports user-mode ELF programs, and provides a tiny shell.

## Architecture

```
Architecture: x86_64
Boot: UEFI
Kernel: monolithic
Language: Rust + assembly
Memory model: higher-half kernel (0xffff_8000_...)
Process model: user/kernel separation
Executable format: ELF64
Filesystem: initramfs -> simple FS
Scheduler: preemptive round-robin
Target emulator: QEMU
```

## Non-goals

- Linux compatibility
- SMP
- dynamic linking
- journaling filesystem
- production-grade security

## Milestones

1. Boot — UEFI bootloader hands off to kernel
2. Kernel logging — serial output works
3. Physical memory — frame allocator
4. Paging — page table management
5. Interrupts — IDT, exceptions, timer
6. Scheduling — kernel threads, preemption
7. User mode — ring 3 entry
8. Syscalls — user-to-kernel interface
9. Initramfs — embedded filesystem
10. ELF loader — spawn user programs
11. Shell — interactive command line
