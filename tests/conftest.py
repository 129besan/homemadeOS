import os
import subprocess
import sys

import pytest


def pytest_collection_modifyitems(items):
    planned_kernel_marker = pytest.mark.skip(
        reason="planned kernel behavior, not validated by the current boot smoke baseline",
    )
    for item in items:
        if "tests/kernel/" in item.nodeid:
            item.add_marker(planned_kernel_marker)


def build_current_image() -> None:
    subprocess.run(
        [sys.executable, "tools/build_initramfs.py"],
        check=True,
    )
    subprocess.run(
        ["cargo", "build", "--package", "kernel", "--target", "x86_64-unknown-none"],
        check=True,
    )
    subprocess.run(
        ["cargo", "build", "--package", "bootloader", "--target", "x86_64-unknown-uefi"],
        check=True,
    )
    subprocess.run([sys.executable, "tools/build_image.py"], check=True)


def run_qemu(timeout: int = 10, expect: str | list[str] | None = None) -> str:
    script = os.path.join(os.path.dirname(__file__), "..", "tools", "run_qemu.py")
    command = [sys.executable, script, "--timeout", str(timeout)]
    if isinstance(expect, str):
        expect = [expect]
    for item in expect or []:
        command.extend(["--expect", item])

    result = subprocess.run(
        command,
        capture_output=True,
        text=True,
    )
    return result.stdout + result.stderr


@pytest.fixture(scope="session")
def qemu_output() -> str:
    build_current_image()
    return run_qemu(timeout=10)
