FROM ubuntu:24.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    qemu-system-x86 \
    dosfstools \
    mtools \
    python3 \
    python3-pip \
    python3-pytest \
    ovmf \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2025-12-01
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-none x86_64-unknown-uefi

WORKDIR /workspace
