#!/usr/bin/env bash
# Enable the versioned git hooks in scripts/hooks/ for this clone.
# Run once after cloning. Uses core.hooksPath so the hooks stay version-controlled
# (no copying into .git/hooks). Local config only — not pushed.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

chmod +x scripts/hooks/* 2>/dev/null || true
git config core.hooksPath scripts/hooks

echo "Hooks enabled: core.hooksPath = scripts/hooks"
echo "Active hooks:"
ls -1 scripts/hooks
