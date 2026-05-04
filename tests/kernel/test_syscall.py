def test_syscall_write(qemu_output):
    assert "write" in qemu_output.lower()


def test_syscall_getpid(qemu_output):
    assert "getpid" in qemu_output.lower()


def test_syscall_spawn(qemu_output):
    assert "spawn" in qemu_output.lower()


def test_syscall_wait(qemu_output):
    assert "wait" in qemu_output.lower()


def test_syscall_fs_read(qemu_output):
    assert "syscall fs read" in qemu_output.lower()
