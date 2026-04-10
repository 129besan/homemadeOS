# Kernel Debugging Guide

## QEMU

```bash
make run
```

This boots the kernel in QEMU with serial output.

## GDB

```bash
qemu-system-x86_64 -s -S -cdrom myos.iso
```

In another terminal:
```bash
gdb target/kernel.elf
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

## Serial Logs

All kernel logs go to COM1 serial port.
QEMU serial output is captured by the test harness.

## Panic

On panic, the kernel prints register state and halts.
Use the symbol lookup helper to decode the instruction pointer:

```bash
python3 tools/symbols.py --addr 0xffff...
```
