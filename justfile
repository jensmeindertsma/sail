help:
    just --list

build:
    cargo build

install: build
    ./development/install.sh

uninstall:
    ./development/uninstall.sh

update: build
    ./development/update.sh

watch: 
    journalctl -f -u sail