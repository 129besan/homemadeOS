from conftest import run_qemu


def test_kernel_starts():
    output = run_qemu(timeout=10)
    assert "kernel started" in output, (
        f"Expected 'kernel started' in QEMU output, got:\n{output}"
    )
