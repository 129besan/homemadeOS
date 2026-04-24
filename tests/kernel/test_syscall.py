import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_syscall_write():
    output = run_qemu(timeout=10, expect="write")
    assert "write" in output.lower()


def test_syscall_getpid():
    output = run_qemu(timeout=10, expect="getpid")
    assert "getpid" in output.lower()


def test_syscall_spawn():
    output = run_qemu(timeout=10, expect="spawn")
    assert "spawn" in output.lower()


def test_syscall_wait():
    output = run_qemu(timeout=10, expect="wait")
    assert "wait" in output.lower()
