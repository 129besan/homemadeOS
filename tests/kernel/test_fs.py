import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "boot"))
from conftest import run_qemu


def test_read_file():
    output = run_qemu(timeout=5)
    assert "open" in output.lower()
