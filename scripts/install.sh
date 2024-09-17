#!/bin/bash

# Set your GitHub repository details
REPO_OWNER="your-username-or-org"
REPO_NAME="your-repository"

# Define the name of the artifact you want to download
ARTIFACT_NAME="your-artifact-name"

# Define the directory where the artifact will be downloaded
DOWNLOAD_DIR="/tmp/sail-installer"

# Create the directory if it doesn't exist
mkdir -p "$DOWNLOAD_DIR"

# Function to get the latest successful workflow run ID on the main branch
get_latest_successful_run_id() {
  curl -s \
    "https://api.github.com/repos/jensmeindertsma/sail/actions/runs?branch=main&status=success" |
    jq -r '.workflow_runs[0].id'
}

# Function to get the artifact download URL from a given workflow run ID
get_artifact_download_url() {
  local run_id=$1
  curl -s \
    "https://api.github.com/repos/jensmeindertsma/sail/actions/runs/$run_id/artifacts" |
    jq -r --arg ARTIFACT_NAME "installer" \
    '.artifacts[] | select(.name == installer) | .archive_download_url'
}

# Function to download the artifact from the given URL
download_artifact() {
  local download_url=$1
  local artifact_file="$DOWNLOAD_DIR/download.zip"
  curl -L -o "$artifact_file" "$download_url"
  echo "Artifact downloaded to $artifact_file"
}

# Main script execution
latest_run_id=$(get_latest_successful_run_id)

if [ -z "$latest_run_id" ]; then
  echo "No successful runs found."
  exit 1
fi

artifact_url=$(get_artifact_download_url "$latest_run_id")

if [ -z "$artifact_url" ]; then
  echo "No artifact found."
  exit 1
fi

download_artifact "$artifact_url"
