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

GROUP="sail"

if getent group "$GROUP" >/dev/null 2>&1; then
    echo "Deleting group '$GROUP'..."
    sudo groupdel "$GROUP"
    echo "Group '$GROUP' deleted."
else
    echo "Group '$GROUP' does not exist."
fi

echo "Done!"
