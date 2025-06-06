#!/bin/bash

set -euo pipefail

echo "Creating new group 'sail'"

if getent group sail >/dev/null 2>&1; then
  echo "Group 'sail' already exists."
else
  sudo groupadd sail
  echo 'Add yourself with sudo usermod -aG docker $USER'
fi

BINARY_PATH="/usr/local/bin"
SERVICE_PATH="/etc/systemd/system"

echo "Copying binaries to '$BINARY_PATH'"

sudo cp "target/debug/sail" "$BINARY_PATH/sail"
sudo cp "target/debug/saild" "$BINARY_PATH/saild"

echo "Copying systemd files to '$SERVICE_PATH'"

sudo cp "deployment/sail.service" "$SERVICE_PATH/sail.service"
sudo cp "deployment/sail.socket" "$SERVICE_PATH/sail.socket"

echo "Reloading systemd"

sudo systemctl daemon-reload

echo "Starting Sail daemon"

sudo systemctl enable --now sail

while ! systemctl is-active --quiet sail; do
  echo "Waiting for Sail daemon to start..."
  sleep 1
done

echo "Done!"
