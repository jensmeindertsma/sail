#!/bin/bash

CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"
SERVICE_PATH="/etc/systemd/system/sail.service"
SOCKET_PATH="/etc/systemd/system/sail.socket"

echo "Removing development installation!"

sudo systemctl stop sail.service sail.socket

sudo rm "$CLI_PATH" "$DAEMON_PATH" "$SERVICE_PATH" "$SOCKET_PATH"

sudo systemctl daemon-reload

echo "Done!"
