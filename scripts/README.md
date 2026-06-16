# scripts/

Tooling for the harness durable layer.

- `build-cli.sh` — builds the Rust Harness CLI and installs it at
  `scripts/bin/harness-cli`. Requires Rust (`brew install rust`).
- `schema/` — version-controlled SQLite schema applied by `harness-cli init`.
- `bin/` — built binary lives here (git-ignored; produced by `build-cli.sh`).
- `hooks/` — version-controlled git hooks. `pre-commit` validates
  `feature_list.json` (valid JSON, ≤1 feature `in_progress`, `passing` features
  have evidence) and runs `harness-cli feature sync`.
- `install-hooks.sh` — enables `hooks/` via `git config core.hooksPath`. Run
  once per clone (also run automatically by `./init.sh`).

## Git hooks

```bash
scripts/install-hooks.sh        # enable the pre-commit hook for this clone
```

The hook blocks a commit only when `feature_list.json` is invalid or violates
the one-`in_progress` / evidence-for-`passing` rules. It writes only to the
git-ignored `harness.db`, never to tracked files. `core.hooksPath` is local
config, so each clone runs `install-hooks.sh` once (or just `./init.sh`).

## Quick start

```bash
scripts/build-cli.sh
scripts/bin/harness-cli init
scripts/bin/harness-cli query stats
```

The CLI expects to run from the **repository root** so it finds `harness.db`
(created in the cwd) and resolves `scripts/schema/` relative to the binary.

## Seeding the durable layer from the docs

The markdown docs are the human-readable seed; mirror them into `harness.db`
after init:

```bash
scripts/bin/harness-cli story add --id US-001 --title "List installed formulae & casks" --lane normal
scripts/bin/harness-cli story add --id US-002 --title "Detect outdated packages" --lane normal
scripts/bin/harness-cli story add --id US-003 --title "List running brew services" --lane normal
scripts/bin/harness-cli story add --id US-004 --title "Show package info / dependencies" --lane normal
scripts/bin/harness-cli story add --id US-005 --title "Upgrade a selected package" --lane high-risk

scripts/bin/harness-cli decision add \
  --id 0001-brew-execution-boundary \
  --title "Single port/adapter for all Homebrew execution" \
  --doc docs/decisions/0001-brew-execution-boundary.md \
  --notes "Accepted at harness bootstrap."

scripts/bin/harness-cli query matrix
```
