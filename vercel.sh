#!/bin/bash

# Set custom directory where binaries will be downloaded and placed in the PATH
BIN_DIR="$HOME/bin"

# Function to download a binary from a URL, rename it, and make it executable
download_and_rename_binary() {
    URL=$1
    ORIGINAL_NAME=$2
    NEW_NAME=$3

    # Download the binary
    curl -L -o "$BIN_DIR/$ORIGINAL_NAME" "$URL"
    
    # Rename the binary
    mv "$BIN_DIR/$ORIGINAL_NAME" "$BIN_DIR/$NEW_NAME"
    
    # Make the binary executable
    chmod +x "$BIN_DIR/$NEW_NAME"
}

# Ensure the custom directory exists
mkdir -p "$BIN_DIR"

# Download fastn binary
fastn_url=$(curl -s "https://api.github.com/repos/fastn-stack/fastn/releases/latest" | grep -oP '"browser_download_url": "\K(https://.*fastn_linux_musl_x86_64)"')
download_and_rename_binary "$fastn_url" "fastn_linux_musl_x86_64" "fastn"

# Download vercel-cache-helper binary
vercel_cache_url=$(curl -s "https://github.com/fastn-stack/vercel-cache-helper/releases/latest" | grep -oP '"browser_download_url": "\K(https://.*vercel-cache-helper_linux_musl_x86_64)"')
download_and_rename_binary "$vercel_cache_url" "vercel-cache-helper_linux_musl_x86_64" "vercel-cache-helper"

# Add the custom directory to your PATH
echo "export PATH=\$PATH:$BIN_DIR" >> "$HOME/.bashrc"
source "$HOME/.bashrc"

# Ensure that the new PATH is loaded
export PATH="$PATH:$BIN_DIR"

# Run vercel-cache-helper download
if command -v vercel-cache-helper &>/dev/null; then
    vercel-cache-helper download
else
    echo "vercel-cache-helper not found in PATH. Please check your PATH configuration."
fi

# Run fastn build
if command -v fastn &>/dev/null; then
    fastn build --edition=2023
else
    echo "fastn not found in PATH. Please check your PATH configuration."
fi

# Run vercel-cache-helper upload
if command -v vercel-cache-helper &>/dev/null; then
    vercel-cache-helper upload
else
    echo "vercel-cache-helper not found in PATH. Please check your PATH configuration."
fi
