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
    bash develop/install.sh

update: build
    bash develop/update.sh

remove:
    bash develop/remove.sh

watch: 
    journalctl --follow --output cat -u sail
