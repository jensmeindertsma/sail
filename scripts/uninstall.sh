#!/bin/bash

set -euo pipefail

BINARY_PATH="/usr/local/bin"
SERVICE_PATH="/etc/systemd/system"

sudo systemctl disable --now sail.socket
sudo systemctl disable --now sail.service

sudo rm "$BINARY_PATH/sail"
sudo rm "$BINARY_PATH/saild"

sudo rm "$SERVICE_PATH/sail.service"
sudo rm "$SERVICE_PATH/sail.socket"

echo "Reloading systemd"

sudo systemctl daemon-reload

echo "Done!"
