# AGENTS.md

Stable agent entrypoint for **brew-mgt**. This file is intentionally small and
stable. It is the universal shim read by Codex, Cursor, and other coding agents.
Claude Code reads `CLAUDE.md`, which points back here. Treat this file as the
contract that does not change often; the operating detail lives in `docs/`.

> The app is what users touch. The harness is what agents touch.

## What This Project Is

`brew-mgt` is a service to **manage tools and services installed by Homebrew** —
inventory installed formulae/casks, detect outdated packages, surface running
`brew services`, and drive safe upgrade/cleanup workflows. Application stack:
**Java / Spring Boot**.

This repository is a **Harness Engineering** Proof of Concept. It follows the
agent-first model where humans steer and agents execute, with the repository as
the system of record. References:

- Learn Harness Engineering — https://walkinglabs.github.io/learn-harness-engineering/en/
- OpenAI: Harness engineering — https://openai.com/index/harness-engineering/
- repository-harness — https://github.com/hoangnb24/repository-harness

## Session Start (do this first, every session)

1. Run `./init.sh` — builds the harness CLI, creates/refreshes `harness.db`,
   syncs `feature_list.json`, runs the baseline verification, prints the start
   command. If verification fails, **stop and fix the baseline** before any
   feature work.
2. Read `claude-progress.md` (Current Verified State + last session) and
   `session-handoff.md`.
3. Read `feature_list.json` — the highest-priority feature that is not `passing`
   is your candidate. Only one feature may be `in_progress` at a time.

## Read First (in order)

1. `docs/HARNESS.md` — the human–agent operating model and task loop.
2. `docs/FEATURE_INTAKE.md` — classify the request and pick a risk lane.
3. `docs/ARCHITECTURE.md` — boundaries and discovery rules.
4. `docs/product/PRODUCT.md` — the current product contract.
5. `docs/TEST_MATRIX.md` — behavior-to-proof expectations.

Deeper docs (routers): `docs/PLANS.md` (exec-plans), `docs/DESIGN.md`,
`docs/SECURITY.md`, `docs/RELIABILITY.md`, `docs/PRODUCT_SENSE.md`,
`docs/design-docs/`, `docs/references/`, and `sops/` for step-by-step recipes.

## The Task Loop (summary)

For every task:

1. Classify the request with `docs/FEATURE_INTAKE.md`.
2. Record the classification: `scripts/bin/harness-cli intake ...`.
3. Mark the chosen feature `in_progress` in `feature_list.json`, then
   `scripts/bin/harness-cli feature sync`.
4. Locate affected product docs and stories.
5. Check proof status: `scripts/bin/harness-cli query matrix`.
6. Work only inside the selected lane: `tiny`, `normal`, or `high-risk`.
   Follow `sops/layered-domain-architecture.md` for feature work.
7. Prove it: run the feature's matrix proofs, record `evidence` in
   `feature_list.json`, mark it `passing` only with evidence, then
   `feature sync`.
8. Before finishing, ask whether product truth, validation expectations,
   architecture rules, failure patterns, or next-agent instructions changed.
   Encode anything new (`sops/encode-unseen-knowledge.md`).
9. Record a trace: `scripts/bin/harness-cli trace ...` (see `docs/TRACE_SPEC.md`).
10. If you hit harness friction, fix it or record it:
    `scripts/bin/harness-cli backlog add ...`.

## Session End (leave a clean state)

Run `clean-state-checklist.md`. At minimum: update `claude-progress.md` and
`session-handoff.md`, ensure `feature_list.json` is honest (no false `passing`),
`feature sync`, record a trace, and confirm `./init.sh` still succeeds. Use
`evaluator-rubric.md` to self-review and `quality-document.md` to track codebase
health over time.

## Durable Layer (Harness CLI)

Operational state — intake classifications, story status, decisions, traces,
and backlog — lives in a local SQLite database (`harness.db`, git-ignored)
managed by the Rust CLI at `scripts/bin/harness-cli`.

Build it once (Rust toolchain required; on macOS: `brew install rust`):

```bash
scripts/build-cli.sh           # builds and installs scripts/bin/harness-cli
scripts/bin/harness-cli init   # create harness.db from scripts/schema/
```

Schema is version-controlled under `scripts/schema/`. The DB is per-clone and
never committed. Full command reference is in `docs/HARNESS.md`.

## Boundaries — Ask a Human First

Agents may update story status/evidence, test-matrix rows, validation notes,
intake records, traces, and backlog items directly. Ask for human confirmation
before: changing architecture direction, removing validation requirements,
changing the source-of-truth hierarchy, changing risk-classification rules, or
replacing the feature workflow.

## Done Definition

A task is done only when: the change is completed or the blocker is documented;
relevant docs, stories, and test-matrix entries are current; validation commands
were run where they exist; a trace was recorded; missing harness capabilities
were logged to the backlog; and the final response states what changed and what
was not attempted.

## Current State

**Comprehensive harness + product docs.** The harness combines the minimal pack
(`init.sh`, `claude-progress.md`, `feature_list.json`), the full template pack
(`session-handoff.md`, `clean-state-checklist.md`, `evaluator-rubric.md`,
`quality-document.md`), the OpenAI Advanced Pack (`docs/exec-plans/`,
`docs/design-docs/`, `docs/references/`, `docs/DESIGN.md`, `docs/SECURITY.md`,
`docs/RELIABILITY.md`, `docs/PRODUCT_SENSE.md`, `docs/PLANS.md`, `sops/`), and a
durable SQLite layer driven by `scripts/bin/harness-cli`. The Homebrew-manager
product contract, seed stories, and test matrix exist. **No Java application
code exists yet** — scaffold the Spring Boot project only when a selected story
requires it (high-risk; needs a decision record — see `docs/ARCHITECTURE.md`).
