def test_kernel_start(qemu_output):
    assert "kernel started" in qemu_output.lower()


def test_memory_init(qemu_output):
    assert "memory" in qemu_output.lower()


def test_timer_tick(qemu_output):
    assert "tick" in qemu_output.lower()
