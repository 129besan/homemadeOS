def test_kernel_start(qemu_output):
    assert "kernel started" in qemu_output.lower()


def test_memory_init(qemu_output):
    assert "memory" in qemu_output.lower()


def test_initramfs_mount(qemu_output):
    assert "initramfs mounted" in qemu_output.lower()


def test_timer_tick(qemu_output):
    assert "tick" in qemu_output.lower()


def test_framebuffer_detected(qemu_output):
    output = qemu_output.lower()
    assert "framebuffer" in output
    assert "framebuffer 0x0" not in output
