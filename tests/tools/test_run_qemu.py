import os
import sys


sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "..", "tools"))
import run_qemu as run_qemu_module


class FakeStdout:
    def __init__(self, lines):
        self._lines = list(lines)
        self._index = 0

    def readline(self):
        if self._index >= len(self._lines):
            return ""
        line = self._lines[self._index]
        self._index += 1
        return line


class FakeProcess:
    def __init__(self, lines):
        self.stdout = FakeStdout(lines)
        self.terminated = False
        self.waited = False
        self.returncode = None

    def poll(self):
        return self.returncode

    def terminate(self):
        self.terminated = True
        self.returncode = 0

    def wait(self, timeout=None):
        self.waited = True
        self.returncode = 0
        return self.returncode


def test_run_qemu_stops_when_expected_output_appears(monkeypatch, capsys):
    process = FakeProcess(["Hello from MyOS!\n", "kernel started\n", "tick\n"])

    monkeypatch.setattr(run_qemu_module.os.path, "exists", lambda path: True)
    monkeypatch.setattr(run_qemu_module.subprocess, "Popen", lambda *args, **kwargs: process)

    output = run_qemu_module.run_qemu(timeout=30, expect=["kernel started"])

    assert "kernel started" in output
    assert process.terminated
    assert process.waited
    assert "kernel started" in capsys.readouterr().out
