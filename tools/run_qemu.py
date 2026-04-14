#!/usr/bin/env python3
import subprocess
import sys
import os
import argparse

def run_qemu(timeout: int = 30):
    img = os.path.join(os.path.dirname(__file__), "..", "build", "uefi.img")

    if not os.path.exists(img):
        print("Image not found, building first...")
        subprocess.run([sys.executable, "tools/build_image.py"], check=True)

    # Find OVMF firmware
    ovmf = None
    for candidate in [
        "/usr/share/OVMF/OVMF_CODE.fd",
        "/usr/share/OVMF/OVMF_CODE_4M.fd",
        "/usr/share/edk2/x64/OVMF_CODE.fd",
    ]:
        if os.path.exists(candidate):
            ovmf = candidate
            break
    if ovmf is None:
        print("OVMF firmware not found, install ovmf package")
        sys.exit(1)

    qemu = [
        "qemu-system-x86_64",
        "-m", "256M",
        "-cpu", "max",
        "-drive", f"if=pflash,format=raw,readonly=on,file={ovmf}",
        "-drive", f"file={img},format=raw,if=ide",
        "-nographic",
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

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--timeout", type=int, default=30)
    args = parser.parse_args()
    run_qemu(args.timeout)
