#!/usr/bin/env bash

set -e

# Allow user to pass custom stats filename (default: stats.csv)
STATS_FILE="${1:-stats.csv}"

# Constants
VENV_DIR=".venv"
PYTHON="$VENV_DIR/bin/python"

# Run the Rust simulation with custom stats file
./target/debug/cars --preset random --ui tui --stats "$STATS_FILE"

# Create Python virtual environment if it doesn't exist
if [ ! -d "$VENV_DIR" ]; then
    echo "[+] Creating Python virtual environment..."
    python3 -m venv "$VENV_DIR"
fi

# Install dependencies silently
echo "[+] Installing Python dependencies..."
"$PYTHON" -m pip install --quiet --upgrade pip
"$PYTHON" -m pip install --quiet pandas matplotlib

# Run the plot script with the specified stats file
echo "[+] Plotting results from $STATS_FILE..."
"$PYTHON" plot.py "$STATS_FILE"

cleanup() {
    echo "[!] Script interrupted. Simulation may not be complete."
    exit 1
}

trap cleanup SIGINT SIGTERM

