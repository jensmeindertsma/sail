help:
    just --list

build:
    cargo build

check:
    cargo clippy --workspace --all-targets --all-features

format:
    cargo fmt --all

install: build
    bash scripts/install.sh
