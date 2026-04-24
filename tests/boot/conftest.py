import subprocess
import sys
import os


def run_qemu(timeout: int = 10, expect: str | list[str] | None = None) -> str:
    script = os.path.join(os.path.dirname(__file__), "..", "..", "tools", "run_qemu.py")
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
