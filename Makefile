.PHONY: all build run clean

all: build

build:
	cargo build --package kernel --target x86_64-unknown-none
	cargo build --package bootloader --target x86_64-unknown-uefi

run: build
	python3 tools/run_qemu.py

clean:
	cargo clean
	rm -rf build/
