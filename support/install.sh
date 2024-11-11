#!/bin/bash

CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"
SERVICE_PATH="/etc/systemd/system/sail.service"
SOCKET_PATH="/etc/systemd/system/sail.socket"
VERSION_PATH="/var/lib/sail/version"

GITHUB_API="https://api.github.com/repos/jensmeindertsma/sail"

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

# We retrieve JSON data about the latest release from GitHub.
# We do not use the `/repos/{owner}/{repo}/releases/latest` endpoint
# because that does not include pre-releases.
LATEST_RELEASE=$(curl -s "$GITHUB_API/releases" | jq -r '[.[]][0]')

# Get the version name from the release
VERSION=$(echo "$LATEST_RELEASE" |  jq -r '.name')

# This is the name of the asset we want to download
ASSET_NAME="sail-$VERSION.tar.gz"
# This is where we place the download temporarily
DOWNLOAD_PATH="/tmp/$ASSET_NAME"
# This is where the download should be extracted to
FILES_PATH="/tmp/sail-$VERSION"

# We look at the release data and pick the asset corresponding to $ASSET_NAME
DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | jq -r ".assets[] | select(.name==\"$ASSET_NAME\") | .browser_download_url")

# Download and unpack assets
curl -s -L "$DOWNLOAD_URL" --output "$DOWNLOAD_PATH"
rm -r "$FILES_PATH" 2> /dev/null
mkdir "$FILES_PATH"
tar -xzf "$DOWNLOAD_PATH" -C "$FILES_PATH"   

# Clean up download file after extracting what we need
rm "$DOWNLOAD_PATH"

echo "moving files, this might ask for sudo permissions"

# Store version for `sail update` to check later
sudo mkdir /etc/sail 2>/dev/null
sudo mkdir /var/lib/sail
echo "$VERSION" | sudo tee "$VERSION_PATH" > /dev/null

# Move files into place
sudo mv "$FILES_PATH/sail" "$CLI_PATH"
sudo mv "$FILES_PATH/saild" "$DAEMON_PATH"
sudo mv "$FILES_PATH/sail.service" "$SERVICE_PATH"
sudo mv "$FILES_PATH/sail.socket" "$SOCKET_PATH"

# Clean up files directory after moving files out of it
rm -r "$FILES_PATH"

# Create group referenced in systemd files *before* reloading systemd
sudo groupadd "sail"

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

read -r -p "Enter registry hostname: " registry_hostname
sudo sail configure registry.hostname "$registry_hostname"

echo "Installation finished!"
echo "** Next steps **"
echo "- configure Nginx to handle TLS and forward all traffic to localhost:4250"
echo "    - if you use Cloudflare, consider using Authenticated Origin Pulls"
echo "- configure UFW to allow HTTPS traffic (and SSH)"
