#!/bin/bash

echo "Updating service binaries!"

CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"

sudo systemctl stop sail

sudo cp "target/debug/sail" "$CLI_PATH"
sudo cp "target/debug/saild" "$DAEMON_PATH"

sudo systemctl start sail

echo "Done!"