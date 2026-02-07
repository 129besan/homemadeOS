# Virtual Memory Layout

## Address Space Layout

```
0x0000_0000_0000_0000 - 0x0000_7fff_ffff_ffff  user space (128 TiB)
0xffff_8000_0000_0000 - 0xffff_8fff_ffff_ffff  direct physical map (64 TiB)
0xffff_9000_0000_0000 - 0xffff_9fff_ffff_ffff  kernel heap (64 TiB)
0xffff_a000_0000_0000 - 0xffff_afff_ffff_ffff  kernel stacks (64 TiB)
0xffff_b000_0000_0000 - 0xffff_bfff_ffff_ffff  MMIO
0xffff_c000_0000_0000 - 0xffff_ffff_ffff_ffff  kernel image / modules
```

## Page Table Levels

x86_64 uses 4-level page tables:

- PML4  (index 39:47)  - 512 entries
- PDPT  (index 30:38)  - 512 entries
- PD    (index 21:29)  - 512 entries
- PT    (index 12:20)  - 512 entries

Each leaf entry maps a 4KiB page.

## Permission Model

| Region       | Supervisor | User |
|-------------|------------|------|
| Kernel text | RX         | -    |
| Kernel data | RW         | -    |
| Direct map  | RW (NX)    | -    |
| User code   | -          | RX   |
| User data   | -          | RW   |
| Null page   | unmapped   | unmapped |
