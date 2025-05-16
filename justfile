help:
    just --list

build:
    cargo build

check:
    cargo clippy

format:
    cargo fmt --all

install: build
    ./scripts/install.sh

uninstall:
    ./scripts/uninstall.sh

update: build
    ./scripts/update.sh

watch: 
    journalctl -f -u sail
