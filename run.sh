#!/bin/bash

# Fake Productivity Runner Script
# Usage: ./run.sh [--matrix] [--quick] [--daemon]

# Set the terminal title
echo -e "\033]0;Fake Productivity Console\007"

echo "🚀 Starting Fake Productivity System..."

# Build the project first
echo "📦 Building application..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo "🎭 Launching fullscreen application..."
echo "   Press Ctrl+C to exit gracefully"
echo

# Run with all passed arguments
./target/release/fake-productivity "$@"
