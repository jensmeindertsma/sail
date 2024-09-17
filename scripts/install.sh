#!/usr/bin/env bash

echo "Installing Sail"

sudo groupadd sail 2>/dev/null

sudo cp ./target/debug/sail /usr/local/bin/sail
sudo cp ./target/debug/saild /usr/local/bin/saild

sudo cp ./scripts/install/systemd.service /usr/lib/systemd/system/sail.service
sudo cp ./scripts/install/systemd.socket /usr/lib/systemd/system/sail.socket

sudo systemctl daemon-reload
sudo systemctl enable --now sail

echo "Please add yourself to the 'sail' group:"
echo "sudo usermod -aG {USER}"