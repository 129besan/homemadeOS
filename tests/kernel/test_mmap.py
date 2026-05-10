def test_syscall_mmap(qemu_output):
    assert "mmap" in qemu_output.lower()
