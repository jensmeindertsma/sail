#!/bin/bash

CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"
SERVICE_PATH="/etc/systemd/system/sail.service"
SOCKET_PATH="/etc/systemd/system/sail.socket"

if
    [ -f "$CLI_PATH"        ]   || 
    [ -f "$DAEMON_PATH"     ]   || 
    [ -f "$SERVICE_PATH"    ]   || 
    [ -f "$SOCKET_PATH"     ]   || 
    [ -f "$VERSION_PATH"    ]
then
    echo "Already installed!"
    exit
fi

if ! systemctl is-active --quiet docker; then
  echo "ERROR: Docker must be installed!"
  exit
fi

echo "Setting up development installation!"

sudo cp "target/debug/sail" "$CLI_PATH"
sudo cp "target/debug/saild" "$DAEMON_PATH"
sudo cp "deployment/sail.service" "$SERVICE_PATH"
sudo cp "deployment/sail.socket" "$SOCKET_PATH"

# Socket cannot be activated until this group exists
sudo groupadd "sail" 2> /dev/null

sudo systemctl daemon-reload
sudo systemctl enable --now sail

while ! systemctl is-active --quiet sail; do
  echo "Waiting for Sail to start..."
  sleep 1
done

echo "Done!"
echo "Run \`sudo usermod -aG sail $YOU\` to get non-sudo access to the CLI"
echo "(then reload your session for it to take effect!)"
