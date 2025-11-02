help:
    just --list

build:
    cargo build

format:
    cargo fmt --check --all

check:
    cargo clippy --workspace

install: build
    bash scripts/install.sh

update: build
    bash scripts/update.sh

remove:
    bash scripts/remove.sh

watch: 
    journalctl --follow --output cat -u sail
