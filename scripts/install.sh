#!/bin/bash

set -euo pipefail

BINARY_PATH="/usr/local/bin"
SERVICE_PATH="/etc/systemd/system"

echo "Copying binaries to '$BINARY_PATH'"

sudo cp "target/debug/sail" "$BINARY_PATH/sail"
sudo cp "target/debug/saild" "$BINARY_PATH/saild"

echo "Copying systemd service file to '$SERVICE_PATH'"

sudo cp "deployment/sail.service" "$SERVICE_PATH/sail.service"

echo "Reloading systemd"

sudo systemctl daemon-reload

echo "Starting saild"

sudo systemctl enable --now sail

while ! systemctl is-active --quiet sail; do
  echo "Waiting for Sail to start..."
  sleep 1
done

echo "Done!"