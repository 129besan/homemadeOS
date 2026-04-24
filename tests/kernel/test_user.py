def test_user_entry(qemu_output):
    assert "user" in qemu_output.lower()
