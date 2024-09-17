help:
    just --list

logs: 
    journalctl --follow --output cat --unit sail

stop:
    sudo systemctl stop sail 2>/dev/null

start:
    sudo systemctl start sail    

upgrade:
    cargo build
    just stop
    sudo cp /home/jens/dev/sail/target/debug/sail /usr/local/bin/sail
    sudo cp /home/jens/dev/sail/target/debug/saild /usr/local/bin/saild
    just start

build-release:
    cargo build --release
