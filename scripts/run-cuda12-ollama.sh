#!/bin/bash
# Wrapper to run dev-voice CUDA binary with Ollama's CUDA 12 libraries
#
# Usage:
#   ./scripts/run-cuda12-ollama.sh daemon
#   ./scripts/run-cuda12-ollama.sh start --duration 10
#
# Or set DEVVOICE_BINARY environment variable:
#   DEVVOICE_BINARY=./path/to/dev-voice ./scripts/run-cuda12-ollama.sh daemon

# Add Ollama's CUDA 12 libs to library path
export LD_LIBRARY_PATH=/usr/local/lib/ollama${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}

# Find dev-voice binary
BINARY="${DEVVOICE_BINARY:-./dev-voice}"

if [ ! -x "$BINARY" ]; then
    # Try common locations
    if [ -x "$HOME/.local/bin/dev-voice-cuda" ]; then
        BINARY="$HOME/.local/bin/dev-voice-cuda"
    elif [ -x "./target/release/dev-voice" ]; then
        BINARY="./target/release/dev-voice"
    elif [ -x "./docs/tmp/dev-voice-linux-x64-cuda/dev-voice" ]; then
        BINARY="./docs/tmp/dev-voice-linux-x64-cuda/dev-voice"
    else
        echo "Error: Cannot find dev-voice binary"
        echo "Set DEVVOICE_BINARY environment variable or install to ~/.local/bin/dev-voice-cuda"
        exit 1
    fi
fi

# Run dev-voice with all arguments passed through
exec "$BINARY" "$@"
