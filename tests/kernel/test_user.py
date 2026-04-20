import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_user_entry():
    output = run_qemu(timeout=10)
    assert "user" in output.lower()
