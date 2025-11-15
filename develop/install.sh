#!/bin/bash

echo "Installing Sail"

if systemctl is-active --quiet sail; then
  echo "quitting! (already installed)"
  exit 1
fi

sudo cp "target/debug/sail" "/usr/local/bin/sail"
sudo cp "target/debug/saild" "/usr/local/bin/saild"

sudo cp "deployment/sail.service" "/etc/systemd/system/sail.service"
sudo cp "deployment/sail.socket" "/etc/systemd/system/sail.socket"

echo "Creating group"

getent group sail >/dev/null || sudo groupadd sail

sudo systemctl daemon-reload
sudo systemctl enable --now sail 2>/dev/null

while ! systemctl is-active --quiet sail; do
  echo "Starting service..."
  sleep 1
done

echo "Adding you to the group"
sudo usermod -aG sail $USER

echo "Done! (please re-open the shell session)"
