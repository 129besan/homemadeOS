# homemadeOS

homemadeOS is a learning OS for x86_64 UEFI boot. It has a Rust `no_std`
kernel, a Rust UEFI bootloader, a small userspace tree, and QEMU smoke tests.

The current baseline boots through UEFI in QEMU, loads the kernel ELF, exits
boot services, passes memory and framebuffer handoff data, and reaches the
kernel serial log message `kernel started`.

## Quick Start

Build the Docker development image and build the OS image:

```bash
make build
```

Run it in QEMU:

```bash
make run
```

Run the current test suite:

```bash
make test
```

For a one-off Docker command, use:

```bash
docker compose run --rm dev python3 -m pytest tests/ -v
```

The currently meaningful boot smoke test is:

```bash
docker compose run --rm dev python3 -m pytest tests/boot/test_boot.py::test_kernel_start -v
```

## Repository Layout

- `bootloader/`: UEFI entry point, ELF loading, memory map conversion, handoff
- `kernel/`: Rust `no_std` kernel, serial logging, memory setup, early subsystems
- `libs/`: shared low-level libraries
- `userspace/`: small user programs used by later milestones
- `tools/`: image builder, initramfs builder, QEMU runner, symbol helper
- `tests/`: QEMU smoke tests and tool tests
- `docs/`: design notes and current state

## Current Test Status

`tests/boot/` verifies the current boot baseline. `tests/tools/` verifies local
test tooling. `tests/kernel/` contains planned behavior tests for later kernel
features and is skipped until each behavior has a reliable implementation-level
baseline.

GitHub Actions workflows are present but manual-only. They run from the Actions
tab via `workflow_dispatch`; pushes and pull requests do not start CI
automatically.

## Documentation

- `docs/000-overview.md`: project shape and milestones
- `docs/013-current-state.md`: current implementation and test baseline
- `docs/011-debugging.md`: QEMU and serial debugging notes
- `docs/012-contributing.md`: branch, commit, and test workflow
