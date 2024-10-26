#!/bin/bash

echo "Updating service binaries!"

CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"

sudo cp "target/debug/sail" "$CLI_PATH"
sudo cp "target/debug/saild" "$DAEMON_PATH"

sudo systemctl restart sail

echo "Done!"