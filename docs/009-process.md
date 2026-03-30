# ELF Process Loader

## ELF64 Loading

1. Validate ELF magic, class, endianness, machine
2. Walk program headers for PT_LOAD segments
3. Map each segment into a new address space
4. Build user stack with argv
5. Create main thread with IP = e_entry

## Process Lifecycle

1. `spawn_elf` reads file from initramfs
2. Creates address space (with kernel mappings copied)
3. Maps ELF segments and user stack
4. Creates thread with `Tid` and `Pid`
5. Enqueues thread in scheduler
6. On exit, process becomes zombie
7. Parent's `wait` reaps zombie

## Limitations

- No fork (no COW)
- No dynamic linking
- No `mmap` file-backed segments
- Read-only ELF from initramfs only
