#!/bin/bash

# Set custom directory where binaries will be downloaded and placed in the PATH
BIN_DIR="$HOME/bin"

# Function to download a binary from a URL and make it executable
download_binary() {
    URL=$1
    BINARY_NAME=$2

    # Download the binary
    curl -L -o "$BIN_DIR/$BINARY_NAME" "$URL"
    
    # Make the binary executable
    chmod +x "$BIN_DIR/$BINARY_NAME"
}

# Ensure the custom directory exists
mkdir -p "$BIN_DIR"

# Download fastn binary
fastn_url=$(curl -s "https://api.github.com/repos/fastn-stack/fastn/releases/latest" | grep -oP '"browser_download_url": "\K(https://.*fastn_linux_musl_x86_64)"')
download_binary "$fastn_url" "fastn_linux_musl_x86_64"

# Download vercel-cache-helper binary
vercel_cache_url=$(curl -s "https://github.com/fastn-stack/vercel-cache-helper/releases/latest" | grep -oP '"browser_download_url": "\K(https://.*vercel-cache-helper_linux_musl_x86_64)"')
download_binary "$vercel_cache_url" "vercel-cache-helper_linux_musl_x86_64"

# Add the custom directory to your PATH
echo "export PATH=\$PATH:$BIN_DIR" >> "$HOME/.bashrc"
source "$HOME/.bashrc"

# Run vercel-cache-helper download
vercel-cache-helper download

# Run fastn build
fastn build --edition=2023

# Run vercel-cache-helper upload
vercel-cache-helper upload
