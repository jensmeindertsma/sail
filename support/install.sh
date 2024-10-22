#!/bin/bash

echo "Downloading latest release"
# - Show download progress
# - Download to /tmp/sail-{commit}

echo "Checking prerequisites"
# - Docker
# - Nginx (correct configuration?)
# - Firewall (UFW)
# - HOW TO: show prompt for each with documentation,
#   then ask user to check box with space and press enter to move on

echo "Moving downloaded files into place"
# - Move CLI and daemon into /usr/local/bin
# - Move systemd files into /usr/lib/systemd/systemd/*
# - Invoke systemctl daemon-reload

echo "Starting systemd service"

echo "Configuring default settings"
# - Ask for hostnames for dashboard and registry
# - Then invoke CLI to configure the settings

echo "Confirming service is online"
# - curl localhost:4250 -H Host:<dashboard_domain>/status
