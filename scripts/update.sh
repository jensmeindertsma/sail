#!/bin/bash

set -euo pipefail

BINARY_PATH="/usr/local/bin"

echo "Shutting down Sail daemon"

sudo systemctl stop sail.service

echo "Copying binaries to '$BINARY_PATH'"

sudo cp "target/debug/sail" "$BINARY_PATH/sail"
sudo cp "target/debug/saild" "$BINARY_PATH/saild"

echo "Restarting Sail daemon"

sudo systemctl restart sail.service

while ! systemctl is-active --quiet sail; do
    echo "Waiting for Sail daemon to start..."
    sleep 1
done

echo "Update complete"
