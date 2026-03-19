#!/usr/bin/env python3
import struct
import os
import sys

MAGIC = b'INITRAMF'

def pack_initramfs(root_dir: str, output: str):
    entries = []
    string_table = b''
    data = b''

    for dirpath, dirnames, filenames in os.walk(root_dir):
        for fname in filenames:
            fpath = os.path.join(dirpath, fname)
            rel = os.path.relpath(fpath, root_dir)
            name_bytes = rel.encode()
            name_offset = len(string_table)
            string_table += name_bytes + b'\x00'

            with open(fpath, 'rb') as f:
                content = f.read()

            data_offset = len(data)
            data += content

            entries.append((name_offset, data_offset, len(content)))

    header_size = 8 + 4 + 4 + 4
    entry_size = 4 + 4 + 4
    string_table_offset = header_size + len(entries) * entry_size
    data_offset = string_table_offset + len(string_table)

    with open(output, 'wb') as f:
        f.write(MAGIC)
        f.write(struct.pack('<I', len(entries)))
        f.write(struct.pack('<I', string_table_offset))
        f.write(struct.pack('<I', data_offset))
        for name_off, data_off, data_len in entries:
            f.write(struct.pack('<III', name_off, data_off, data_len))
        f.write(string_table)
        f.write(data)

    print(f"packed {len(entries)} files into {output}")

if __name__ == '__main__':
    pack_initramfs(sys.argv[1], sys.argv[2])
