help:
    just --list

build:
    cargo build

check:
    cargo clippy

format:
    cargo fmt --all
