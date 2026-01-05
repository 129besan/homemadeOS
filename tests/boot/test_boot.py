import subprocess
import sys
import os


def run_qemu(timeout: int = 10) -> str:
    script = os.path.join(os.path.dirname(__file__), "..", "..", "tools", "run_qemu.py")
    result = subprocess.run(
        [sys.executable, script, "--timeout", str(timeout)],
        capture_output=True,
        text=True,
    )
    return result.stdout + result.stderr


def test_kernel_starts():
    output = run_qemu(timeout=10)
    assert "kernel started" in output, (
        f"Expected 'kernel started' in QEMU output, got:\n{output}"
    )
