#!/bin/bash

if ! systemctl is-active --quiet sail; then
  echo "quitting! (not installed)"
  exit 1
fi

echo "Removing Sail"

sudo systemctl disable --now sail.socket sail.service 2>/dev/null

while systemctl is-active --quiet sail; do
  echo "Waiting for shutdown..."
  sleep 1
done

sudo rm "/usr/local/bin/sail"
sudo rm "/usr/local/bin/saild"

sudo rm "/etc/systemd/system/sail.service"
sudo rm "/etc/systemd/system/sail.socket"

echo "Removing group"

sudo groupdel "sail"

sudo systemctl daemon-reload

echo "Done!"
