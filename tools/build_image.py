#!/usr/bin/env python3
import os
import subprocess
import tempfile
import argparse

BUILD_DIR = os.path.join(os.path.dirname(__file__), "..", "build")


def build_fat_image(size_mb: int = 64):
    os.makedirs(BUILD_DIR, exist_ok=True)
    img_path = os.path.join(BUILD_DIR, "uefi.img")

    subprocess.run(
        ["dd", "if=/dev/zero", f"of={img_path}", "bs=1M", f"count={size_mb}"],
        check=True,
        capture_output=True,
    )
    subprocess.run(
        ["mkfs.fat", "-F32", img_path],
        check=True,
        capture_output=True,
    )
    print(f"Created FAT image: {img_path}")
    return img_path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--size", type=int, default=64)
    args = parser.parse_args()
    build_fat_image(args.size)


if __name__ == "__main__":
    main()
