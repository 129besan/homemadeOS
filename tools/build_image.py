#!/usr/bin/env python3
import os
import subprocess
import sys

BUILD_DIR = os.path.join(os.path.dirname(__file__), "..", "build")
PROJECT_ROOT = os.path.join(os.path.dirname(__file__), "..")

def build_image(size_mb: int = 64):
    os.makedirs(BUILD_DIR, exist_ok=True)
    img_path = os.path.join(BUILD_DIR, "uefi.img")

    subprocess.run(
        ["dd", "if=/dev/zero", f"of={img_path}", "bs=1M", f"count={size_mb}"],
        check=True, capture_output=True,
    )
    subprocess.run(
        ["mkfs.fat", "-F32", img_path],
        check=True, capture_output=True,
    )

    bootloader_src = os.path.join(
        PROJECT_ROOT, "target", "x86_64-unknown-uefi", "debug", "bootloader.efi"
    )

    subprocess.run(["mmd", "-i", img_path, "::EFI"], check=True)
    subprocess.run(["mmd", "-i", img_path, "::EFI/BOOT"], check=True)
    subprocess.run(
        ["mcopy", "-i", img_path, bootloader_src, "::EFI/BOOT/BOOTX64.EFI"],
        check=True,
    )

    print(f"Boot image ready: {img_path}")
    return img_path

if __name__ == "__main__":
    build_image()
