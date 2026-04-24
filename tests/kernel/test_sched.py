def test_two_threads(qemu_output):
    assert "thread" in qemu_output.lower()
