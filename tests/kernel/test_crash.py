import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_user_crash():
    output = run_qemu(timeout=10, expect=["page fault", "killed"])
    assert "page fault" in output.lower() or "killed" in output.lower()
