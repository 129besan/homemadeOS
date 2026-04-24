def test_spawn(qemu_output):
    assert "spawn" in qemu_output.lower()
