#!/usr/bin/env python3
import subprocess
import sys
import os
import argparse


def build_image():
    subprocess.run(
        ["python3", "tools/build_image.py"],
        check=True,
    )


def run_qemu(image: str, timeout: int = 30):
    qemu = [
        "qemu-system-x86_64",
        "-m", "256M",
        "-cpu", "max",
        "-drive", f"if=pflash,format=raw,file={image}",
        "-serial", "stdio",
        "-display", "none",
        "-no-reboot",
    ]
    if timeout:
        qemu += ["-no-shutdown"]
        try:
            subprocess.run(qemu, timeout=timeout)
        except subprocess.TimeoutExpired:
            pass
    else:
        subprocess.run(qemu)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--timeout", type=int, default=30)
    args = parser.parse_args()

    image = os.path.join(os.path.dirname(__file__), "..", "build", "uefi.img")
    if not os.path.exists(image):
        build_image()

    run_qemu(image, args.timeout)


if __name__ == "__main__":
    main()
