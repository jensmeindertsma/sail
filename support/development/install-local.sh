#!/bin/bash

CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"
SERVICE_PATH="/etc/systemd/system/sail.service"
SOCKET_PATH="/etc/systemd/system/sail.socket"
VERSION_PATH="/var/lib/sail/version"

# Before doing anything, let's check if there are already files on the disk that
# indicate that Sail is already installed. 
if
    [ -f "$CLI_PATH"        ]   && 
    [ -f "$DAEMON_PATH"     ]   && 
    [ -f "$SERVICE_PATH"    ]   && 
    [ -f "$SOCKET_PATH"     ]   && 
    [ -f "$VERSION_PATH"    ]
then
    echo "Already installed!"
    exit
fi

# We check if Docker is installed & running because that is a hard
# requirement for Sail to function. Whether Nginx / UFW is configured
# correctly we do not check, because it is optional. We do point users
# in the right direction by asking them to configure said services
# at the end of the installer.
if ! systemctl is-active --quiet docker; then
  echo "error: docker is not running! this is a prerequisite for Sail"
  exit
fi 

echo "copying files, this might ask for sudo permissions"

sudo mkdir /var/lib/sail
# TODO: figure out version for local installs

# Move files into place
sudo cp "target/debug/sail" "$CLI_PATH"
sudo cp "target/debug/saild" "$DAEMON_PATH"
sudo cp "support/systemd/sail.service" "$SERVICE_PATH"
sudo cp "support/systemd/sail.socket" "$SOCKET_PATH"

sudo groupadd "sail" 2> /dev/null

# Reload systemd for it to pick up the new service files
sudo systemctl daemon-reload

# Finally, get things running
sudo systemctl enable sail
sudo systemctl start sail

# Wait until the service is active
while ! systemctl is-active --quiet sail; do
  echo "Waiting for Sail to start..."
  sleep 1
done

echo "Sail is now running."

read -r -p "Enter dashboard hostname: " dashboard_hostname
sudo sail configure dashboard.hostname "$dashboard_hostname"

read -r -p "Enter registry hostname: " registry_hostname
sudo sail configure registry.hostname "$registry_hostname"

curl -H "Host: $dashboard_hostname" localhost:4250 /status 

echo "Installation finished!"
echo "** Next steps **"
echo "- configure Nginx to handle TLS and forward all traffic to localhost:4250"
echo "    - if you use Cloudflare, consider using Authenticated Origin Pulls"
echo "- configure UFW to allow HTTPS traffic (and SSH)"
