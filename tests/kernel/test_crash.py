def test_user_crash(qemu_output):
    assert "killed" in qemu_output.lower()
