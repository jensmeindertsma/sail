set quiet

help:
    just --list

build:
    cargo build

clean:
  cargo clean

check:
  cargo clippy --workspace

format:
  cargo fmt --all

format-ci:
  cargo fmt --all --check

install: build
    bash scripts/install.sh

update: build
    bash scripts/update.sh

remove:
    bash scripts/remove.sh

watch: 
    journalctl --follow --output cat -u sail
