#!/usr/bin/env python3
import subprocess
import sys
import os
import argparse
import select
import time


def _wait_for_output(process, timeout: int, expect: list[str] | None) -> str:
    output = []
    deadline = time.monotonic() + timeout if timeout else None
    expected = [item.lower() for item in expect or []]

    while True:
        line = ""
        stdout = process.stdout
        if stdout is None:
            break

        if hasattr(stdout, "fileno"):
            wait = 0.1
            if deadline is not None:
                wait = max(0.0, min(wait, deadline - time.monotonic()))
            ready, _, _ = select.select([stdout], [], [], wait)
            if ready:
                line = stdout.readline()
        else:
            line = stdout.readline()

        if line:
            output.append(line)
            print(line, end="", flush=True)
            current = "".join(output).lower()
            if expected and any(item in current for item in expected):
                process.terminate()
                process.wait(timeout=2)
                break
            continue

        if process.poll() is not None:
            break

        if deadline is not None and time.monotonic() >= deadline:
            process.terminate()
            process.wait(timeout=2)
            break

        if not hasattr(stdout, "fileno"):
            break

    return "".join(output)


def run_qemu(timeout: int = 30, expect: list[str] | None = None) -> str:
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

    if timeout or expect:
        qemu += ["-no-shutdown"]
        process = subprocess.Popen(
            qemu,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
        )
        return _wait_for_output(process, timeout, expect)
    else:
        subprocess.run(qemu)
        return ""

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--timeout", type=int, default=30)
    parser.add_argument("--expect", action="append", default=None)
    args = parser.parse_args()
    run_qemu(args.timeout, args.expect)
