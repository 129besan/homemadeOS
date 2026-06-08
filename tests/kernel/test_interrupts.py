def test_timer_interrupt(qemu_output):
    assert "tick" in qemu_output.lower() or "timer" in qemu_output.lower()


def test_breakpoint(qemu_output):
    assert "breakpoint" in qemu_output.lower()
