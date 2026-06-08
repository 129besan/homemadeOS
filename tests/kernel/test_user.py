def test_user_entry(qemu_output):
    assert "user=true" in qemu_output.lower()
