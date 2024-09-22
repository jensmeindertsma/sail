help:
    just --list

logs: 
    journalctl --follow --output cat --unit sail

stop:
    sudo systemctl stop sail 2>/dev/null

start:
    sudo systemctl start sail    

install: stop
    cargo build
    sudo cp /home/jens/dev/sail/target/debug/sail /usr/local/bin/sail
    sudo cp /home/jens/dev/sail/target/debug/saild /usr/local/bin/saild
    just start

update:
    cargo upgrade --incompatible --pinned
    cargo clean
    cargo check
    just upgrade

start-abc-server:
    cargo run --bin abc_server