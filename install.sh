#!/bin/bash

echo "Making directories..."
mkdir -p ~/.config/refresh

echo "Building refresh..."
cargo build --release

echo "Installing refresh..."
sudo mv target/release/refresh /usr/local/bin/refresh

echo "Installing uninstall script..."
sudo cp src/uninstall.sh ~/.config/refresh/uninstall.sh

echo "Cleaning up..."
rm -rf ~/.cargo ~/.rustup

echo "Installation complete!"
