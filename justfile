help:
    just --list

build:
    cargo build

check:
    cargo clippy

install: build
    sudo cp "target/debug/sail" "/usr/local/bin/sail"

update: build
    sudo cp "target/debug/sail" "/usr/local/bin/sail"

uninstall:
    sudo rm "/usr/local/bin/sail"
