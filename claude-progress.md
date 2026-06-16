# Progress Log

The single source of truth for where the project stands. **Every session reads
this first and updates it last.** Newest session entry on top.

## Current Verified State

- **Repository root:** the `brew-mgt` project directory.
- **Standard startup path:** `./init.sh` (builds harness CLI, inits DB, syncs
  features, runs baseline verification, prints start command).
- **Standard verification path:** `VERIFY_CMD` in `init.sh` — currently `true`
  (no app code yet). Becomes `./gradlew test` after the Spring Boot scaffold.
- **Highest priority unfinished feature:** `US-001` — List installed formulae &
  casks (see `feature_list.json`).
- **Current blocker:** none. App is not scaffolded; scaffolding is the first
  high-risk step and needs a decision record (see `docs/ARCHITECTURE.md`).

## Session Record

### 2026-06-15 — Harness bootstrap (comprehensive)

- **Goal:** Stand up a comprehensive Harness Engineering structure for brew-mgt
  that works for both Claude Code and Codex.
- **Completed:** Agent entrypoints (`AGENTS.md`/`CLAUDE.md`); harness policy docs
  (`docs/HARNESS.md`, `FEATURE_INTAKE.md`, `ARCHITECTURE.md`, `TEST_MATRIX.md`,
  `TRACE_SPEC.md`, `HARNESS_BACKLOG.md`); templates; product contract under
  `docs/product/`; seed stories; decision 0001; Rust `harness-cli` + SQLite;
  minimal pack (`init.sh`, this log, `feature_list.json`); full template pack
  (handoff, clean-state checklist, evaluator rubric, quality document); OpenAI
  Advanced Pack (exec-plans, design-docs, references, RELIABILITY/SECURITY,
  PLANS, PRODUCT_SENSE, SOPs); `feature sync` CLI command.
- **Verification run:** none mechanical (no app, and Rust toolchain not present
  in the authoring sandbox). SQL is standard SQLite DDL; Rust reviewed by hand.
- **Evidence recorded:** n/a (no passing app tests yet).
- **Commits:** files staged; commit to be made on the developer machine.
- **Known risks:** the Rust CLI has not yet been compiled; run
  `scripts/build-cli.sh` to confirm it builds.
- **Next best action:** run `./init.sh`, then either scaffold the Spring Boot
  project (high-risk; write a decision record) or pick up `US-001`.
