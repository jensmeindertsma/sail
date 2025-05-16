#!/bin/bash

CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"

echo "Updating development installation!"

sudo systemctl stop sail.service sail.socket

sudo cp "target/debug/sail" "$CLI_PATH"
sudo cp "target/debug/saild" "$DAEMON_PATH"

sudo systemctl start sail

echo "Done!"
