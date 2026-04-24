def test_open(qemu_output):
    assert "open" in qemu_output.lower()


def test_read(qemu_output):
    assert "read" in qemu_output.lower()


def test_close(qemu_output):
    assert "close" in qemu_output.lower()


def test_enoent(qemu_output):
    assert "enoent" in qemu_output.lower()
