def test_shell_prompt(qemu_output):
    assert "$ " in qemu_output
