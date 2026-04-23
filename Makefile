.PHONY: all build run clean dev-up dev-shell test test-one

ifeq ($(INSIDE_DOCKER),)
dev-up:
	docker compose up -d dev

dev-shell: dev-up
	docker compose exec dev bash

test: dev-up
	docker compose exec dev python3 -m pytest tests/ -v

test-one: dev-up
	docker compose exec dev python3 -m pytest $(TEST) -v

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

test:
	python3 -m pytest tests/ -v

test-one:
	python3 -m pytest $(TEST) -v

run: build
	python3 tools/run_qemu.py

clean:
	cargo clean
	rm -rf build/
endif
