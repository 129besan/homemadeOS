import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_kernel_start():
    output = run_qemu(timeout=15, expect="kernel started")
    assert "kernel started" in output.lower()


def test_memory_init():
    output = run_qemu(timeout=10, expect="memory")
    assert "memory" in output.lower()


def test_timer_tick():
    output = run_qemu(timeout=10, expect="tick")
    assert "tick" in output.lower()
