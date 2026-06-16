# CLAUDE.md

This file is Claude Code's entrypoint. The full, tool-agnostic operating
contract lives in **[`AGENTS.md`](./AGENTS.md)** — read it first. Everything
there applies to Claude Code identically.

## Quick orientation for Claude Code

- This is `brew-mgt`: a Java / Spring Boot service to manage Homebrew-installed
  tools and services. It is also a Harness Engineering PoC.
- The repository is the system of record. Read `docs/` before changing anything.
- Operational state lives in SQLite via `scripts/bin/harness-cli` (build with
  `scripts/build-cli.sh`), **not** in chat history.

## Session start

1. Run `./init.sh` (builds CLI, inits/refreshes `harness.db`, syncs features,
   runs baseline verification). If it fails, fix the baseline first.
2. Read `claude-progress.md` and `session-handoff.md`.
3. Read `feature_list.json`; pick the highest-priority non-`passing` feature.

## Required reading order

1. [`AGENTS.md`](./AGENTS.md) — entrypoint, session flow, task loop.
2. [`docs/HARNESS.md`](./docs/HARNESS.md) — operating model + CLI reference.
3. [`docs/FEATURE_INTAKE.md`](./docs/FEATURE_INTAKE.md) — risk lanes.
4. [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md) — boundaries.
5. [`docs/product/PRODUCT.md`](./docs/product/PRODUCT.md) — product contract.
6. [`docs/TEST_MATRIX.md`](./docs/TEST_MATRIX.md) — proof expectations.

Routers into deeper docs: [`docs/PLANS.md`](./docs/PLANS.md),
[`docs/DESIGN.md`](./docs/DESIGN.md), [`docs/SECURITY.md`](./docs/SECURITY.md),
[`docs/RELIABILITY.md`](./docs/RELIABILITY.md),
[`docs/PRODUCT_SENSE.md`](./docs/PRODUCT_SENSE.md), and [`sops/`](./sops/).

## Session end

Run [`clean-state-checklist.md`](./clean-state-checklist.md); update the progress
log + handoff; keep `feature_list.json` honest and run `harness-cli feature
sync`; record a trace.

## Do not

- Do not edit code before classifying the request via `docs/FEATURE_INTAKE.md`.
- Do not claim validation passed for commands that do not exist yet.
- Do not commit `harness.db` (it is git-ignored and per-clone).
- Do not change architecture, validation requirements, or risk rules without
  human confirmation (see the boundaries section in `AGENTS.md`).

When in doubt, follow `AGENTS.md`. If `AGENTS.md` and this file ever disagree,
`AGENTS.md` wins.
