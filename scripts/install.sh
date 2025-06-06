#!/bin/bash

set -euo pipefail

echo "Making sure the 'sail' group is ready to go!"

GROUP="sail"

if getent group "$GROUP" >/dev/null 2>&1; then
  echo "Group '$GROUP' already exists."

  if id -nG "$USER" | grep -qw "$GROUP"; then
    echo "User '$USER' is already in the '$GROUP' group."
  else
    echo "User '$USER' is not in the '$GROUP' group. Adding..."
    sudo usermod -aG "$GROUP" "$USER"
    echo "User '$USER' added to '$GROUP'. You may need to log out and log back in."
  fi

else
  echo "Group '$GROUP' does not exist. Creating..."
  sudo groupadd "$GROUP"
  echo "Adding user '$USER' to '$GROUP'..."
  sudo usermod -aG "$GROUP" "$USER"
  echo "User '$USER' added to '$GROUP'. You may need to log out and log back in."
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
