def test_user_crash(qemu_output):
    assert "page fault" in qemu_output.lower() or "killed" in qemu_output.lower()
