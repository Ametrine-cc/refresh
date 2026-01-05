#!/bin/bash

# Uninstall script for refresh

# Remove the binary from the system path
sudo rm -f /usr/local/bin/refresh

# Remove the directory
rm -rf ~/.config/refresh

# Remove the uninstall script
sudo rm -f ~/.config/refresh/uninstall.sh

# Remove the directory

echo "refresh has been uninstalled successfully."
