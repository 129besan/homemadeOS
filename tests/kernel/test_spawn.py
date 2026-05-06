def test_spawn(qemu_output):
    assert "spawn" in qemu_output.lower()


def test_spawn_elf_loaded(qemu_output):
    assert "spawn elf loaded" in qemu_output.lower()
