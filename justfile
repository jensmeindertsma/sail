help:
    just --list

build:
    cargo build

update: build
    ./support/development/update.sh

uninstall:
    sudo sail uninstall 2>/dev/null || true

install: uninstall
    ./support/development/install-local.sh