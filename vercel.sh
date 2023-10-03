#!/bin/bash

# Function to download and install fastn
install_fastn() {
  echo "🚀 Installing fastn..."
  curl -fsSL https://raw.githubusercontent.com/ftd-lang/fastn/main/install.sh | sh
  echo "✅ fastn installed successfully!"
} 

# Function to download vercel-cache-helper and add it to a user-specific directory
install_vercel_cache_helper() {
  local install_dir="$HOME/bin"
  mkdir -p "$install_dir"

  # Download vercel-cache-helper using curl
  echo "🚀 Downloading vercel-cache-helper..."
  curl -fsSL https://github.com/fastn-stack/vercel-cache-helper/releases/latest/download/vercel-cache-helper_linux_musl_x86_64 -o "$install_dir/vercel-cache-helper"

  # Make it executable
  chmod +x "$install_dir/vercel-cache-helper"

  # Add to the PATH
  export PATH="$PATH:$install_dir"

  # Check if it's in the PATH
  command -v vercel-cache-helper >/dev/null 2>&1 || {
    echo "❌ vercel-cache-helper not found in PATH. Please check your installation."
    exit 1
  }
  echo "✅ vercel-cache-helper installed successfully!"
}

# Main script
set -e  # Exit on any error

install_fastn
install_vercel_cache_helper

echo "🚀 Downloading cache using vercel-cache-helper..."
vercel-cache-helper download

# Build with fastn
echo "🚀 Building with fastn..."
fastn build "$@"
echo "✅ Build completed successfully!"

echo "🚀 Uploading cache using vercel-cache-helper..."
vercel-cache-helper upload
echo "✅ Cache uploaded successfully!"
