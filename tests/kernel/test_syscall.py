import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_syscall_write():
    output = run_qemu(timeout=10)
    assert "write" in output.lower()


def test_syscall_getpid():
    output = run_qemu(timeout=10)
    assert "getpid" in output.lower()


def test_syscall_spawn():
    output = run_qemu(timeout=10)
    assert "spawn" in output.lower()


def test_syscall_wait():
    output = run_qemu(timeout=10)
    assert "wait" in output.lower()
