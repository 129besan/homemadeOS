import sys
sys.path.insert(0, "tests/boot")
from conftest import run_qemu


def test_timer_interrupt():
    output = run_qemu(timeout=10, expect=["tick", "timer"])
    assert "tick" in output.lower() or "timer" in output.lower()


def test_breakpoint():
    output = run_qemu(timeout=10, expect="breakpoint")
    assert "breakpoint" in output.lower()
