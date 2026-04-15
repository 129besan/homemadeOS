# Current State

This repo has a Docker-based OS build and QEMU test environment.

## Branching

The old work was done directly on `main`. New work should use focused
`feature/*` branches.

The old `main` was a known broken baseline. The current feature branch has a
passing kernel-entry smoke test.

## What Works

- Docker image installs Rust nightly, QEMU, OVMF, mtools, and pytest.
- Bootloader builds for `x86_64-unknown-uefi`.
- Kernel builds for the custom x86_64 target.
- Disk image generation works through `tools/build_image.py`.
- QEMU boots through UEFI.
- Bootloader prints `Hello from MyOS!`.
- Bootloader loads a static kernel ELF.
- Kernel serial output reaches `kernel started`.

## Current Passing Check

The first boot smoke test now passes:

```bash
docker compose run --rm dev python3 -m pytest tests/boot/test_boot.py::test_kernel_start -v
```

Expected output includes:

```text
kernel started
```

## Remaining Broken Areas

- `ExitBootServices` is not wired correctly yet.
- BootInfo memory map fields are placeholders.
- Framebuffer fields are placeholders.
- Most later smoke tests still describe planned behavior, not working behavior.

## What Was Fixed

The original failure had several causes:

- Workspace builds were not applying kernel linker flags.
- The kernel ELF was being produced as PIE/DYN instead of a static executable.
- The bootloader read embedded ELF headers through potentially unaligned typed
  references.
- The UEFI bootloader called the kernel with the UEFI/Win64 ABI instead of the
  kernel's SysV ABI.
- The serial driver used memory volatile access for I/O ports instead of x86
  `in`/`out` instructions.

## Useful Commands

Build and test through Docker:

```bash
docker compose run --rm dev python3 -m pytest tests/ -v
```

Inspect the kernel ELF inside Docker:

```bash
docker compose run --rm dev readelf -h target/x86_64-unknown-none/debug/kernel
docker compose run --rm dev readelf -l target/x86_64-unknown-none/debug/kernel
```

Rebuild the image after rebuilding kernel or bootloader:

```bash
docker compose run --rm dev python3 tools/build_image.py
```

## Key Files

- `bootloader/src/main.rs`
- `bootloader/src/elf_loader.rs`
- `bootloader/src/handoff.rs`
- `kernel/src/main.rs`
- `kernel/linker.ld`
- `kernel/x86_64-myos.json`
- `kernel/.cargo/config.toml`
- `tools/build_image.py`
- `tools/run_qemu.py`
- `tests/boot/test_boot.py`
