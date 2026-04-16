.PHONY: all build run clean

ifeq ($(INSIDE_DOCKER),)
build:
	docker compose run --rm dev make build INSIDE_DOCKER=1

run:
	docker compose run --rm dev make run INSIDE_DOCKER=1

clean:
	docker compose run --rm dev make clean INSIDE_DOCKER=1
else
build:
	cargo build --package kernel --target x86_64-unknown-none
	cargo build --package bootloader --target x86_64-unknown-uefi

run: build
	python3 tools/run_qemu.py

clean:
	cargo clean
	rm -rf build/
endif
