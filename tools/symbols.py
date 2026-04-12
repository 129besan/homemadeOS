#!/usr/bin/env python3
import subprocess
import sys

def lookup_symbol(addr: str, elf: str = "target/kernel.elf"):
    try:
        result = subprocess.run(
            ["addr2line", "-e", elf, "-f", "-p", addr],
            capture_output=True, text=True,
        )
        print(result.stdout.strip())
    except FileNotFoundError:
        print(f"error: addr2line not found")
        print(f"suggestion: addr2line -e {elf} -f -p {addr}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("usage: symbols.py --addr <hex_addr>")
        sys.exit(1)
    addr = sys.argv[2] if len(sys.argv) > 2 else sys.argv[1]
    if addr.startswith("0x"):
        addr = addr[2:]
    lookup_symbol(addr)
