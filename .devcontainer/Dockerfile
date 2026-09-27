# McLoad Rust Dev Container — stable toolchain (see rust-toolchain.toml)
FROM rust:bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libssl-dev \
        git \
        curl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Align with repo rust-toolchain.toml (stable) and common editor tooling.
RUN rustup default stable \
    && rustup component add rustfmt clippy rust-analyzer

WORKDIR /workspace
