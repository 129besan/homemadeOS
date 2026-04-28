#!/usr/bin/env python3
import shutil
import subprocess
import sys
from pathlib import Path

from mkinitramfs import pack_initramfs

ROOT = Path(__file__).resolve().parent.parent
TARGET = "x86_64-unknown-none"
PROGRAMS = ("hello", "echo", "shell", "init", "ls", "cat")


def cargo_build_userspace() -> None:
    command = ["cargo", "build", "--target", TARGET]
    for package in PROGRAMS:
        command.extend(["--package", package])
    subprocess.run(command, cwd=ROOT, check=True)


def copy_programs(root_dir: Path) -> None:
    bin_dir = root_dir / "bin"
    bin_dir.mkdir(parents=True, exist_ok=True)

    target_dir = ROOT / "target" / TARGET / "debug"
    for program in PROGRAMS:
        source = target_dir / program
        destination = root_dir / "init" if program == "init" else bin_dir / program
        shutil.copy2(source, destination)


def main() -> int:
    build_dir = ROOT / "build"
    root_dir = build_dir / "initramfs-root"
    output = build_dir / "initramfs.img"

    cargo_build_userspace()
    if root_dir.exists():
        shutil.rmtree(root_dir)
    root_dir.mkdir(parents=True)
    copy_programs(root_dir)
    pack_initramfs(str(root_dir), str(output))
    return 0


if __name__ == "__main__":
    sys.exit(main())
