#!/bin/bash

# Set the terminal title
echo -e "\033]0;Fake Productivity Console\007"

echo "Starting Fake Productivity System..."
echo "Use --matrix flag for Matrix mode: ./run.sh --matrix"
echo

cargo run -- "$@"
