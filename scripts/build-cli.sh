#!/usr/bin/env bash
# Build the Harness CLI and install it at scripts/bin/harness-cli.
# Requires a Rust toolchain (on macOS: `brew install rust`).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found. Install Rust first (macOS: brew install rust)." >&2
  exit 1
fi

echo "Building harness-cli (release)..."
cargo build --release --package harness-cli

mkdir -p "$REPO_ROOT/scripts/bin"
BIN_SRC="$REPO_ROOT/target/release/harness-cli"
BIN_DST="$REPO_ROOT/scripts/bin/harness-cli"
cp "$BIN_SRC" "$BIN_DST"
chmod +x "$BIN_DST"

echo "Installed: scripts/bin/harness-cli"
echo "Next: scripts/bin/harness-cli init"
