def test_echo_command(qemu_output):
    assert "echo" in qemu_output.lower()
