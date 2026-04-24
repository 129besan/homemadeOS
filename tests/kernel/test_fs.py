import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_open():
    output = run_qemu(timeout=10, expect="open")
    assert "open" in output.lower()


def test_read():
    output = run_qemu(timeout=10, expect="read")
    assert "read" in output.lower()


def test_close():
    output = run_qemu(timeout=10, expect="close")
    assert "close" in output.lower()


def test_enoent():
    output = run_qemu(timeout=10, expect="enoent")
    assert "enoent" in output.lower()
