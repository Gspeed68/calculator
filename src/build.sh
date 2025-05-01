#!/bin/bash

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Build the project
echo "Building the project..."
wasm-pack build --target web

echo "Build complete! You can now open index.html in your browser." 