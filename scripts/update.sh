#!/bin/bash

if ! systemctl is-active --quiet sail; then
  echo "quitting! (not installed)"
  exit 1
fi

echo "Updating installation"

echo "Stopping service..."

sudo systemctl stop sail.socket sail.service

sudo cp "target/debug/sail" "/usr/local/bin/sail"
sudo cp "target/debug/saild" "/usr/local/bin/saild"

sudo cp "deployment/sail.service" "/etc/systemd/system/sail.service"
sudo cp "deployment/sail.socket" "/etc/systemd/system/sail.socket"

 echo "Restarting service..."

sudo systemctl daemon-reload
sudo systemctl restart sail

while ! systemctl is-active --quiet sail; do
  echo "Waiting for startup..."
  sleep 1
done

echo "Done!"
