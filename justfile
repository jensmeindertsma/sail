help:
    just --list

build:
    cargo build --release

check:
    cargo clippy

install: build
    sudo cp "target/release/sail" "/usr/local/bin/sail"

update: build
    sudo cp "target/release/sail" "/usr/local/bin/sail"

uninstall:
    sudo rm "/usr/local/bin/sail"
