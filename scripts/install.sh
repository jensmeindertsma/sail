#!/bin/bash

echo "Installing Sail"

sudo cp "target/debug/sail" "/usr/local/bin/sail"
sudo cp "target/debug/saild" "/usr/local/bin/saild"
