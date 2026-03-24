# VFS and Initramfs

## VFS Architecture

- `FileOps` trait: read, write, seek
- `InodeOps` trait: open, lookup
- `FileTable`: per-process file descriptor table

## Initramfs

The initial filesystem is embedded in the kernel image as an initramfs archive.

### Format

```
[header] [entry table] [string table] [file data]

header:
  magic: [u8; 8] = "INITRAMF"
  file_count: u32
  string_table_offset: u32
  data_offset: u32

entry:
  name_offset: u32
  data_offset: u32
  data_len: u32
```

### Mounting

During kernel init, `mount_root(boot_info)` parses the initramfs and creates the root filesystem.

## Device Nodes

- `/dev/console`: write-only output to serial console
- `/dev/null`: accepts all writes
