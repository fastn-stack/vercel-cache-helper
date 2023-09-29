#!/bin/bash

# Function to download and install fastn
install_fastn() {
  curl -fsSL https://raw.githubusercontent.com/ftd-lang/fastn/main/install.sh | sh
} 

# Function to download vercel-cache-helper and add it to a user-specific directory
install_vercel_cache_helper() {
  local install_dir="$HOME/bin" # Change this to your desired installation directory
  mkdir -p "$install_dir"

  # Download vercel-cache-helper using curl
  curl -fsSL https://github.com/fastn-stack/vercel-cache-helper/releases/latest/download/vercel-cache-helper_linux_musl_x86_64 -o "$install_dir/vercel-cache-helper"

  # Make it executable
  chmod +x "$install_dir/vercel-cache-helper"

  # Add to the PATH
  export PATH="$PATH:$install_dir"

  # Check if it's in the PATH
  command -v vercel-cache-helper >/dev/null 2>&1 || {
    echo "vercel-cache-helper not found in PATH. Please check your installation."
    exit 1
  }
}

# Main script
install_fastn
install_vercel_cache_helper

vercel-cache-helper download

# Build with fastn
fastn build --edition=2023

vercel-cache-helper upload
