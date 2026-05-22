def test_syscall_mmap(qemu_output):
    assert "mmap" in qemu_output.lower()


def test_syscall_mmap_distinct(qemu_output):
    assert "mmap distinct" in qemu_output.lower()
