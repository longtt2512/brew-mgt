# brew-mgt

A service to **manage tools and services installed by Homebrew** — inventory
installed formulae/casks, detect outdated packages, surface running
`brew services`, and drive safe upgrade/cleanup workflows.

Stack: **Java / Spring Boot**. This repository is also a **Harness Engineering**
Proof of Concept following the agent-first model (humans steer, agents execute,
the repository is the system of record).

> The app is what users touch. The harness is what agents touch.

## For agents (Claude Code & Codex)

Start at **[`AGENTS.md`](./AGENTS.md)** (Codex/Cursor/others) or
**[`CLAUDE.md`](./CLAUDE.md)** (Claude Code) — both point to the same operating
model. Do not edit code or docs before reading them and classifying the request
via [`docs/FEATURE_INTAKE.md`](./docs/FEATURE_INTAKE.md).

## Repository layout

```text
brew-mgt/
  AGENTS.md                 # universal agent entrypoint (Codex, Cursor, ...)
  CLAUDE.md                 # Claude Code entrypoint -> points to AGENTS.md
  README.md
  init.sh                   # session-start: build CLI, init DB, sync, verify
  feature_list.json         # feature tracker (source of truth) -> harness.db
  claude-progress.md        # progress log (read first / write last each session)
  session-handoff.md        # compact handoff to the next session
  clean-state-checklist.md  # end-of-session checklist
  evaluator-rubric.md       # score agent output (0-2 x 6 dimensions)
  quality-document.md       # codebase health over time (A-D)
  docs/
    HARNESS.md              # operating model + CLI reference
    FEATURE_INTAKE.md       # input types + risk lanes (tiny/normal/high-risk)
    ARCHITECTURE.md         # boundaries & discovery rules
    TEST_MATRIX.md          # behavior-to-proof (durable copy in harness.db)
    TRACE_SPEC.md           # how to record execution traces
    HARNESS_BACKLOG.md      # harness growth backlog
    DESIGN.md SECURITY.md RELIABILITY.md PRODUCT_SENSE.md PLANS.md   # policy docs
    product/                # the brew-mgt product contract (no code yet)
    product-specs/          # spec-style write-ups (router into product/)
    stories/                # story packets (US-001 ...)
    decisions/              # durable decision records
    design-docs/            # durable reasoning / core beliefs
    exec-plans/             # active/ + completed/ + tech-debt-tracker.md
    references/             # captured external refs (brew --json shapes, ...)
    templates/              # spec / story / decision / validation / exec-plan
  sops/                     # step-by-step operating procedures
  scripts/
    build-cli.sh            # build + install the Harness CLI
    schema/                 # version-controlled SQLite schema (0001, 0002)
    README.md
  crates/harness-cli/       # Rust Harness CLI (clap + rusqlite + serde_json)
  Cargo.toml                # Rust workspace
```

## Current state

**Harness v0 + product docs.** The harness, the Homebrew-manager product
contract, seed stories, and the test matrix exist. **No Java application code
exists yet** — it is scaffolded only when a selected story requires it (a
high-risk, decision-recorded step; see `docs/ARCHITECTURE.md`).

## Harness CLI (durable layer)

Operational state lives in a local, git-ignored SQLite DB managed by a small
Rust CLI. Requires a Rust toolchain (macOS: `brew install rust`).

```bash
./init.sh                       # one shot: build CLI, init DB, sync features, verify
scripts/bin/harness-cli query matrix
```

`./init.sh` is the standard session-start path. Under the hood it runs
`scripts/build-cli.sh`, `harness-cli init`, and `harness-cli feature sync`. You
can also run those individually.

See [`scripts/README.md`](./scripts/README.md) for seeding commands and
[`docs/HARNESS.md`](./docs/HARNESS.md) for the full command reference.

## References

- Learn Harness Engineering — https://walkinglabs.github.io/learn-harness-engineering/en/
- OpenAI: Harness engineering — https://openai.com/index/harness-engineering/
- repository-harness — https://github.com/hoangnb24/repository-harness
