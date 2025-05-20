#!/usr/bin/env bash
set -e

# Positional args
RULE="${1:-B3/S23}"                     # Default to B3/S23 if not provided
STATS_FILE="${2:-stats.csv}"           # Default to stats.csv if not provided

# Environment
VENV_DIR=".venv"
PYTHON="$VENV_DIR/bin/python"

echo "[+] Running rule: $RULE"
echo "[+] Saving stats to: $STATS_FILE"
echo "[+] Logging output to: $LOG_FILE"

# Run the simulation and capture output
${CARGO_TARGET_DIR:-target}/debug/cars \
  --preset random \
  --rule "$RULE" \
  --ui tui \
  --stats "$STATS_FILE" \

# Setup Python venv if needed
if [ ! -d "$VENV_DIR" ]; then
    echo "[+] Creating Python virtual environment..."
    python3 -m venv "$VENV_DIR"
fi

echo "[+] Installing Python dependencies..."
"$PYTHON" -m pip install --quiet --upgrade pip
"$PYTHON" -m pip install --quiet pandas matplotlib

echo "[+] Plotting results..."
"$PYTHON" plot.py "$STATS_FILE"

