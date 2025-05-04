help:
    just --list

build:
    cargo build

check:
    cargo clippy

format:
    cargo fmt --all

install: build
    ./development/install.sh

uninstall:
    ./development/uninstall.sh

update: build
    ./development/update.sh

watch: 
    journalctl -f -u sail