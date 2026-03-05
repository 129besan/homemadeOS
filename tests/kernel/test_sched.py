import sys
sys.path.insert(0, "tests/boot")
from conftest import run_qemu


def test_two_threads():
    output = run_qemu(timeout=5)
    assert "thread" in output.lower()
