#!/bin/bash

ARTIFACT_NAME="sail-release"
CLI_PATH="/usr/local/bin/sail"
DAEMON_PATH="/usr/local/bin/saild"
SERVICE_PATH="/etc/systemd/system/sail.service"
SOCKET_PATH="/etc/systemd/system/sail.socket"
VERSION_PATH="/var/lib/sail/version"

GITHUB_API="https://api.github.com/repos/jensmeindertsma/sail"

### STEP 1 ### -> check if Sail is already installed
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

### STEP 2 ### -> download latest release from GitHub
echo "(1) Downloading latest release"
echo "(1a) fetching latest succesful release workflow details"

LATEST_RELEASE=$(curl -s "$GITHUB_API/releases" | jq -r '[.[]][0]')
LATEST_VERSION=$(echo "$LATEST_RELEASE" |  jq -r '.name')
DOWNLOAD_NAME="sail-$LATEST_VERSION"
DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | jq -r '.assets[] | select(.name==\"\") | .id .assets[0].browser_download_url')

if [ -z "$LATEST_RUN_ID" ]; then
  echo "error: no successful workflow run found!"
  exit 1
fi

echo "latest run ID: $LATEST_RUN_ID"
echo "latest version: $LATEST_VERSION"

echo "(1b) fetching release artifact produced by this workflow"

ARTIFACTS=$(curl -s "$GITHUB_API/actions/runs/$LATEST_RUN_ID/artifacts")
ARTIFACT_ID=$(echo "$ARTIFACTS" | jq -r ".artifacts[] | select(.name==\"$ARTIFACT_NAME\") | .id")

if [ -z "$ARTIFACT_ID" ]; then
  echo "error: latest succesful workflow run did not produce new release files!"
  exit 1
fi

echo "artifact ID: $ARTIFACT_ID"

DOWNLOAD_ZIP_PATH="/tmp/sail-$LATEST_VERSION.zip"
DOWNLOAD_PATH="/tmp/sail-$LATEST_VERSION"

curl -s -L "$GITHUB_API/actions/artifacts/$ARTIFACT_ID/zip" --output "$DOWNLOAD_ZIP_PATH"
# python3 -c "import zipfile; zipfile.ZipFile('$DOWNLOAD_ZIP_PATH', 'r').extractall('$DOWNLOAD_PATH')"
# rm "$DOWNLOAD_ZIP_PATH"

### STEP 3 ###
echo "Checking prerequisites"
# - Docker, check if installed & running with docker info or similar

echo "Moving downloaded files into place"
# - Store version into /var/lib/sail/version
# - Move CLI and daemon into /usr/local/bin
# - Move systemd files into /usr/lib/systemd/systemd/*
# - Invoke systemctl daemon-reload
# - Delete tmp download folder

echo "Starting systemd service"
# - wait for status is now running

echo "Configuring default settings"
# - Ask for hostnames for dashboard and registry
# - Then invoke CLI to configure the settings
# - `sail configure dashboard.hostname hello.world`

echo "Confirming service is online"
# - curl localhost:4250 -H Host:<dashboard_domain>/status

echo "Please configure your firewall and proxy"
# - Point the user to the latest documentation (on Github) explaining how
# to configure UFW and Nginx to handle TLS and forward all requests.
# - Add special section on Cloudflare Origin Certificates and their client pull
# verification how to enable that in Nginx
