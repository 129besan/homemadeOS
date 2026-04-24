import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_echo_command():
    output = run_qemu(timeout=10, expect="echo")
    assert "echo" in output.lower()
