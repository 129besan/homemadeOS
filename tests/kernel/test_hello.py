def test_hello(qemu_output):
    assert "hello" in qemu_output.lower()
