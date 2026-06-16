#!/usr/bin/env bash
# init.sh — fixed session-start script for brew-mgt.
#
# One shot: confirm location, build/verify the harness durable layer, install
# app dependencies (when the app exists), run the baseline verification, and
# print the start command. If verification fails, STOP and fix the baseline
# before doing any feature work.
#
# Edit the three command variables below as the project grows.
set -euo pipefail

# ----- project commands (edit these) ----------------------------------------
# App dependency install. No app code yet, so this is a no-op placeholder.
INSTALL_CMD="${INSTALL_CMD:-true}"
# Baseline verification. Becomes e.g. './gradlew test' once the app exists.
VERIFY_CMD="${VERIFY_CMD:-true}"
# Dev server start. Becomes e.g. './gradlew bootRun' once the app exists.
START_CMD="${START_CMD:-echo \"(no app yet — scaffold via a story first)\"}"
# -----------------------------------------------------------------------------

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

echo "==> brew-mgt init"
echo "    repo root: $REPO_ROOT"

# 1. Harness durable layer: build the CLI and create harness.db if needed.
echo "==> Harness durable layer"
if command -v cargo >/dev/null 2>&1; then
  if [ ! -x scripts/bin/harness-cli ]; then
    echo "    building harness-cli..."
    scripts/build-cli.sh
  fi
  if [ ! -f harness.db ]; then
    echo "    initializing harness.db..."
    scripts/bin/harness-cli init
  fi
  echo "    syncing feature_list.json into harness.db..."
  scripts/bin/harness-cli feature sync || echo "    (feature sync skipped)"
else
  echo "    WARNING: cargo not found — harness CLI unavailable."
  echo "    Install Rust (macOS: brew install rust), then re-run init.sh."
fi

# 2. App dependencies.
echo "==> Installing app dependencies"
eval "$INSTALL_CMD"

# 3. Baseline verification.
echo "==> Running baseline verification"
if ! eval "$VERIFY_CMD"; then
  echo "ERROR: baseline verification failed. Fix the baseline before feature work." >&2
  exit 1
fi

# 4. Start command.
echo "==> Start command:"
echo "    $START_CMD"
if [ "${RUN_START_COMMAND:-0}" = "1" ]; then
  eval "$START_CMD"
fi

echo "==> init complete. Read claude-progress.md and feature_list.json before starting."
